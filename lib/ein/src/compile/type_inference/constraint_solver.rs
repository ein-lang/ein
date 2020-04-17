use super::super::error::CompileError;
use super::super::reference_type_resolver::ReferenceTypeResolver;
use super::subsumption_set::SubsumptionSet;
use super::variable_constraint_set::VariableConstraintSet;
use crate::types::Type;
use std::collections::HashMap;

#[derive(Debug)]
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
            match (subsumption.lower(), subsumption.upper()) {
                (lower, Type::Variable(variable)) => {
                    if let Type::Variable(lower) = lower {
                        if variable.id() == lower.id() {
                            continue;
                        }
                    }

                    for type_ in variable_constraint_set.get_upper_types(variable.id()) {
                        subsumption_set.add_subsumption(lower.clone(), type_.clone());
                    }

                    variable_constraint_set.add_lower_type(variable, lower);
                }
                (Type::Variable(variable), upper) => {
                    for type_ in variable_constraint_set.get_lower_types(variable.id()) {
                        subsumption_set.add_subsumption(type_.clone(), upper.clone());
                    }

                    variable_constraint_set.add_upper_type(variable, upper);
                }
                (Type::Reference(reference), other) => subsumption_set.add_subsumption(
                    self.reference_type_resolver.resolve_reference(reference)?,
                    other.clone(),
                ),
                (one, Type::Reference(reference)) => subsumption_set.add_subsumption(
                    one.clone(),
                    self.reference_type_resolver.resolve_reference(reference)?,
                ),
                (Type::Function(one), Type::Function(other)) => {
                    subsumption_set
                        .add_subsumption(other.argument().clone(), one.argument().clone());
                    subsumption_set.add_subsumption(one.result().clone(), other.result().clone());
                }
                (Type::Union(union), other) => {
                    for type_ in union.types() {
                        subsumption_set.add_subsumption(type_.clone(), other.clone());
                    }
                }
                (one, Type::Union(union)) => {
                    // Union types are inferred already before inference.
                    if union.types().iter().find(|type_| type_ == &one).is_none() {
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
                    };
                }
                (lower, upper) => {
                    return Err(CompileError::TypesNotMatched(
                        lower.source_information().clone(),
                        upper.source_information().clone(),
                    ))
                }
            }
        }

        Ok(variable_constraint_set.to_substitutions())
    }
}
