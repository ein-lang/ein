use super::super::error::CompileError;
use super::super::reference_type_resolver::ReferenceTypeResolver;
use super::super::type_equality_checker::TypeEqualityChecker;
use super::subsumption_set::SubsumptionSet;
use super::variable_constraint_set::VariableConstraintSet;
use crate::types::Type;
use std::collections::HashMap;
use std::sync::Arc;

pub struct ConstraintSolver {
    reference_type_resolver: Arc<ReferenceTypeResolver>,
    type_equality_checker: Arc<TypeEqualityChecker>,
}

impl ConstraintSolver {
    pub fn new(
        reference_type_resolver: Arc<ReferenceTypeResolver>,
        type_equality_checker: Arc<TypeEqualityChecker>,
    ) -> Self {
        Self {
            reference_type_resolver,
            type_equality_checker,
        }
    }

    pub fn solve(
        &self,
        mut subsumption_set: SubsumptionSet,
    ) -> Result<HashMap<usize, Type>, CompileError> {
        let mut variable_constraint_set = VariableConstraintSet::new();

        while let Some(subsumption) = subsumption_set.remove() {
            match subsumption {
                (lower, Type::Variable(variable)) => {
                    if let Type::Variable(lower) = &lower {
                        if variable.id() == lower.id() {
                            continue;
                        }
                    }

                    for type_ in variable_constraint_set.get_upper_types(variable.id()) {
                        subsumption_set.add(lower.clone(), type_.clone());
                    }

                    variable_constraint_set.add_lower_type(&variable, &lower);
                }
                (Type::Variable(variable), upper) => {
                    for type_ in variable_constraint_set.get_lower_types(variable.id()) {
                        subsumption_set.add(type_.clone(), upper.clone());
                    }

                    variable_constraint_set.add_upper_type(&variable, &upper);
                }
                (Type::Reference(reference), other) => subsumption_set.add(
                    self.reference_type_resolver.resolve_reference(&reference)?,
                    other.clone(),
                ),
                (one, Type::Reference(reference)) => subsumption_set.add(
                    one.clone(),
                    self.reference_type_resolver.resolve_reference(&reference)?,
                ),
                (Type::Function(one), Type::Function(other)) => {
                    subsumption_set.add(other.argument().clone(), one.argument().clone());
                    subsumption_set.add(one.result().clone(), other.result().clone());
                }
                (Type::List(one), Type::List(other)) => {
                    subsumption_set.add(one.element().clone(), other.element().clone());
                }
                (_, Type::Any(_)) => {}
                (Type::Union(union), other) => {
                    for type_ in union.types() {
                        subsumption_set.add(type_.clone(), other.clone());
                    }
                }
                (one, Type::Union(union)) => {
                    // Union types' members cannot be type variables.
                    if !union
                        .types()
                        .iter()
                        .map(|type_| self.type_equality_checker.equal(&one, type_))
                        .collect::<Result<Vec<_>, _>>()?
                        .into_iter()
                        .any(|value| value)
                    {
                        return Err(CompileError::TypesNotMatched(
                            one.source_information().clone(),
                            union.source_information().clone(),
                        ));
                    }
                }
                (Type::Boolean(_), Type::Boolean(_)) => {}
                (Type::None(_), Type::None(_)) => {}
                (Type::Number(_), Type::Number(_)) => {}
                (Type::Record(one), Type::Record(other)) => {
                    if one.name() != other.name() {
                        return Err(CompileError::TypesNotMatched(
                            one.source_information().clone(),
                            other.source_information().clone(),
                        ));
                    }
                }
                (one, other) => {
                    return Err(CompileError::TypesNotMatched(
                        one.source_information().clone(),
                        other.source_information().clone(),
                    ));
                }
            }
        }

        Ok(variable_constraint_set.to_substitutions())
    }
}
