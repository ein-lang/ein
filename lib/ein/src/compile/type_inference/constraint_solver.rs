use super::super::error::CompileError;
use super::super::reference_type_resolver::ReferenceTypeResolver;
use super::subsumption::Subsumption;
use super::subsumption_set::SubsumptionSet;
use crate::types::{self, Type};
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
        let mut substitutions = HashMap::<usize, Type>::new();

        while let Some(subsumption) = subsumption_set.remove() {
            match (subsumption.lower(), subsumption.upper()) {
                (lower, Type::Variable(variable)) => {
                    if let Type::Variable(lower) = lower {
                        if variable.id() == lower.id() {
                            continue;
                        }
                    }

                    for type_ in variable.upper_types() {
                        subsumption_set.add_subsumption(lower.clone(), type_.clone());
                    }

                    self.substitute_variable(
                        variable,
                        &variable.add_lower_type(lower.clone()).into(),
                        &mut substitutions,
                        &mut subsumption_set,
                    );
                }
                (Type::Variable(variable), upper) => {
                    for type_ in variable.lower_types() {
                        subsumption_set.add_subsumption(type_.clone(), upper.clone());
                    }

                    self.substitute_variable(
                        variable,
                        &variable.add_upper_type(upper.clone()).into(),
                        &mut substitutions,
                        &mut subsumption_set,
                    );
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

        Ok(substitutions)
    }

    fn substitute_variable(
        &self,
        variable: &types::Variable,
        type_: &Type,
        substitutions: &mut HashMap<usize, Type>,
        subsumption_set: &mut SubsumptionSet,
    ) {
        for (_, substituted_type) in substitutions.iter_mut() {
            *substituted_type = substituted_type
                .clone()
                .substitute_variable(variable, type_);
        }

        for subsumption in subsumption_set.iter_mut() {
            *subsumption = Subsumption::new(
                subsumption.lower().substitute_variable(variable, type_),
                subsumption.upper().substitute_variable(variable, type_),
            )
        }

        substitutions.insert(variable.id(), type_.clone());
    }
}
