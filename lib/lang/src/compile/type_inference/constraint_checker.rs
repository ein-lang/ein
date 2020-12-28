use super::super::error::CompileError;
use super::super::reference_type_resolver::ReferenceTypeResolver;
use super::super::type_equality_checker::TypeEqualityChecker;
use super::subsumption_set::SubsumptionSet;
use super::variable_substitutor::VariableSubstitutor;
use crate::types::Type;
use std::sync::Arc;

pub struct ConstraintChecker {
    variable_substitutor: Arc<VariableSubstitutor>,
    reference_type_resolver: Arc<ReferenceTypeResolver>,
    type_equality_checker: Arc<TypeEqualityChecker>,
}

impl ConstraintChecker {
    pub fn new(
        variable_substitutor: Arc<VariableSubstitutor>,
        reference_type_resolver: Arc<ReferenceTypeResolver>,
        type_equality_checker: Arc<TypeEqualityChecker>,
    ) -> Self {
        Self {
            variable_substitutor,
            reference_type_resolver,
            type_equality_checker,
        }
    }

    pub fn check(&self, mut subsumption_set: SubsumptionSet) -> Result<(), CompileError> {
        while let Some(subsumption) = subsumption_set.remove() {
            match (
                self.variable_substitutor.substitute(&subsumption.0)?,
                self.variable_substitutor.substitute(&subsumption.1)?,
            ) {
                (_, Type::Any(_)) => {}
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
                (Type::Union(one), Type::Union(other)) => {
                    for type_ in one.types() {
                        subsumption_set.add(type_.clone(), other.clone());
                    }
                }
                (one, Type::Union(union)) => {
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
                (Type::String(_), Type::String(_)) => {}
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

        Ok(())
    }
}
