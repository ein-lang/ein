use super::super::error::CompileError;
use super::super::reference_type_resolver::ReferenceTypeResolver;
use super::subsumption_set::SubsumptionSet;
use super::variable_constraint_set::VariableConstraintSet;
use crate::types::Type;
use std::collections::HashMap;

pub struct ConstraintSolver<'a> {
    reference_type_resolver: &'a ReferenceTypeResolver,
}

impl<'a> ConstraintSolver<'a> {
    pub fn new(reference_type_resolver: &'a ReferenceTypeResolver) -> Self {
        Self {
            reference_type_resolver,
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
                (Type::Union(union), other) => {
                    for type_ in union.types() {
                        subsumption_set.add(type_.clone(), other.clone());
                    }
                }
                (one, Type::Union(union)) => {
                    // Union types' members cannot be type variables.
                    if !union.types().iter().any(|type_| type_ == &one) {
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
                    ))
                }
            }
        }

        Ok(variable_constraint_set.to_substitutions())
    }
}
