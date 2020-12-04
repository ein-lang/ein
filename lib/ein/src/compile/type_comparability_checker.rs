use super::error::CompileError;
use super::reference_type_resolver::ReferenceTypeResolver;
use crate::types::Type;
use std::collections::HashSet;
use std::sync::Arc;

pub struct TypeComparabilityChecker {
    reference_type_resolver: Arc<ReferenceTypeResolver>,
}

impl TypeComparabilityChecker {
    pub fn new(reference_type_resolver: impl Into<Arc<ReferenceTypeResolver>>) -> Arc<Self> {
        Self {
            reference_type_resolver: reference_type_resolver.into(),
        }
        .into()
    }

    pub fn check(&self, type_: &Type) -> Result<bool, CompileError> {
        self.check_with_cache(type_, &Default::default())
    }

    fn check_with_cache(
        &self,
        type_: &Type,
        record_names: &HashSet<String>,
    ) -> Result<bool, CompileError> {
        Ok(match type_ {
            Type::Any(_) => false,
            Type::Boolean(_) => true,
            Type::Function(_) => false,
            Type::List(list) => self.check_with_cache(list.element(), record_names)?,
            Type::None(_) => true,
            Type::Number(_) => true,
            Type::Record(record) => {
                if record_names.contains(record.name()) {
                    true
                } else {
                    let mut record_names = record_names.clone();

                    record_names.insert(record.name().into());

                    record
                        .elements()
                        .values()
                        .map(|type_| self.check_with_cache(type_, &record_names))
                        .collect::<Result<Vec<_>, _>>()?
                        .into_iter()
                        .all(|flag| flag)
                }
            }
            Type::Reference(reference) => self.check_with_cache(
                &self.reference_type_resolver.resolve_reference(reference)?,
                record_names,
            )?,
            Type::String(_) => true,
            Type::Union(union) => union
                .types()
                .iter()
                .map(|type_| self.check_with_cache(type_, record_names))
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .all(|flag| flag),
            Type::Unknown(_) | Type::Variable(_) => unreachable!(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use crate::debug::*;
    use crate::types;

    #[test]
    fn check_comparability_of_record_type() {
        assert!(
            TypeComparabilityChecker::new(ReferenceTypeResolver::new(&Module::dummy()))
                .check(
                    &types::Record::new("foo", Default::default(), SourceInformation::dummy())
                        .into()
                )
                .unwrap()
        );
    }

    #[test]
    fn check_comparability_of_record_type_with_function_member() {
        assert!(
            !TypeComparabilityChecker::new(ReferenceTypeResolver::new(&Module::dummy()))
                .check(
                    &types::Record::new(
                        "foo",
                        vec![(
                            "foo".into(),
                            types::Function::new(
                                types::Number::new(SourceInformation::dummy()),
                                types::Number::new(SourceInformation::dummy()),
                                SourceInformation::dummy(),
                            )
                            .into()
                        )]
                        .into_iter()
                        .collect(),
                        SourceInformation::dummy()
                    )
                    .into()
                )
                .unwrap()
        );
    }

    #[test]
    fn check_comparability_of_record_type_with_any_member() {
        assert!(
            !TypeComparabilityChecker::new(ReferenceTypeResolver::new(&Module::dummy()))
                .check(
                    &types::Record::new(
                        "foo",
                        vec![(
                            "foo".into(),
                            types::Any::new(SourceInformation::dummy()).into()
                        )]
                        .into_iter()
                        .collect(),
                        SourceInformation::dummy()
                    )
                    .into()
                )
                .unwrap()
        );
    }
}
