use super::error::CompileError;
use super::reference_type_resolver::ReferenceTypeResolver;
use crate::types::Type;

pub struct TypeEqualityChecker<'a> {
    reference_type_resolver: &'a ReferenceTypeResolver,
}

impl<'a> TypeEqualityChecker<'a> {
    pub fn new(reference_type_resolver: &'a ReferenceTypeResolver) -> Self {
        Self {
            reference_type_resolver,
        }
    }

    pub fn equal(&self, one: &Type, other: &Type) -> Result<bool, CompileError> {
        let one = self.reference_type_resolver.resolve(&one)?;
        let other = self.reference_type_resolver.resolve(&other)?;

        let value = match (&one, &other) {
            (Type::Record(one), Type::Record(other)) => one.name() == other.name(),
            (Type::Union(one), Type::Union(other)) => {
                one.types().len() == other.types().len()
                    && one
                        .types()
                        .iter()
                        .zip(other.types())
                        .map(|(one, other)| self.equal(one, other))
                        .collect::<Result<Vec<_>, CompileError>>()?
                        .iter()
                        .all(|value| *value)
            }
            (_, _) => one == other,
        };

        Ok(value)
    }
}

#[cfg(tests)]
mod tests {
    use super::*;
    use crate::ast::*;

    #[test]
    fn equal() {
        let record_type = types::Record::new("Foo", vec![], SourceInformation::dummy());
        let reference_type = types::Reference::new("Foo", SourceInformation::dummy());

        let module = Module::with_definitions_and_type_definitions(
            vec![TypeDefinition::new("Foo", record_type.clone())],
            vec![],
        );
        let reference_type_resolver = ReferenceTypeResolver::new(&module);
        let type_equality_checker = TypeEqualityChecker::new(&reference_type_resolver);

        for ((one, other), result) in &[
            (
                (
                    types::Boolean::new(SourceInformation::dummy()),
                    types::Boolean::new(SourceInformation::dummy()),
                ),
                true,
            ),
            (
                (
                    types::Boolean::new(SourceInformation::dummy()),
                    types::None::new(SourceInformation::dummy()),
                ),
                false,
            ),
            ((reference_type.clone(), reference_type.clone()), true),
            ((reference_type.clone(), record_type.clone()), true),
            ((record_type.clone(), reference_type.clone()), true),
            ((record_type.clone(), record_type.clone()), true),
            (
                (
                    types::Union::new(
                        vec![
                            types::Boolean::new(SourceInformation::dummy()),
                            types::None::new(SourceInformation::dummy()),
                        ],
                        SourceInformation::dummy(),
                    ),
                    types::Union::new(
                        vec![
                            types::Boolean::new(SourceInformation::dummy()),
                            types::None::new(SourceInformation::dummy()),
                        ],
                        SourceInformation::dummy(),
                    ),
                ),
                true,
            ),
            (
                (
                    types::Union::new(
                        vec![
                            types::Boolean::new(SourceInformation::dummy()),
                            types::None::new(SourceInformation::dummy()),
                            types::Number::new(SourceInformation::dummy()),
                        ],
                        SourceInformation::dummy(),
                    ),
                    types::Union::new(
                        vec![
                            types::Boolean::new(SourceInformation::dummy()),
                            types::None::new(SourceInformation::dummy()),
                        ],
                        SourceInformation::dummy(),
                    ),
                ),
                false,
            ),
            (
                (
                    types::Union::new(
                        vec![
                            types::None::new(SourceInformation::dummy()),
                            record_type.clone(),
                        ],
                        SourceInformation::dummy(),
                    ),
                    types::Union::new(
                        vec![
                            types::None::new(SourceInformation::dummy()),
                            reference_type.clone(),
                        ],
                        SourceInformation::dummy(),
                    ),
                ),
                true,
            ),
        ] {
            assert_eq!(type_equality_checker.equal(one, other), result);
        }
    }
}
