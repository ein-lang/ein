use super::error::CompileError;
use super::reference_type_resolver::ReferenceTypeResolver;
use super::type_equality_checker::TypeEqualityChecker;
use crate::types::{self, Type};
use std::sync::Arc;

pub struct TypeCanonicalizer {
    reference_type_resolver: Arc<ReferenceTypeResolver>,
    type_equality_checker: Arc<TypeEqualityChecker>,
}

impl TypeCanonicalizer {
    pub fn new(
        reference_type_resolver: Arc<ReferenceTypeResolver>,
        type_equality_checker: Arc<TypeEqualityChecker>,
    ) -> Arc<Self> {
        Self {
            reference_type_resolver,
            type_equality_checker,
        }
        .into()
    }

    pub fn canonicalize(&self, type_: &Type) -> Result<Type, CompileError> {
        type_.convert_types(&mut |type_| self.canonicalize_shallowly(type_))
    }

    fn canonicalize_shallowly(&self, type_: &Type) -> Result<Type, CompileError> {
        Ok(if let Type::Union(union) = type_ {
            self.canonicalize_union_shallowly(&union)?
        } else {
            type_.clone()
        })
    }

    // This function assumes that contained types are canonicalized already.
    fn canonicalize_union_shallowly(&self, union: &types::Union) -> Result<Type, CompileError> {
        let all_types = self.get_member_types(union)?;

        if let Some(type_) = all_types.iter().find(|type_| type_.is_any()) {
            Ok(type_.clone())
        } else {
            let mut types = vec![];

            'outer: for type_ in &all_types {
                for other in &types {
                    if self.type_equality_checker.equal(type_, other)? {
                        continue 'outer;
                    }
                }

                types.push(type_.clone());
            }

            Ok(match types.len() {
                0 => unreachable!(),
                1 => types[0].clone(),
                _ => types::Union::new(types, union.source_information().clone()).into(),
            })
        }
    }

    fn get_member_types(&self, union: &types::Union) -> Result<Vec<Type>, CompileError> {
        Ok(union
            .types()
            .iter()
            .map(|type_| {
                Ok(match self.reference_type_resolver.resolve(type_)? {
                    Type::Union(union) => self.get_member_types(&union)?,
                    // Do not use resolved types because they might not be canonicalized yet.
                    _ => vec![type_.clone()],
                })
            })
            .collect::<Result<Vec<_>, CompileError>>()?
            .into_iter()
            .flatten()
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use crate::debug::SourceInformation;
    use crate::types;
    use pretty_assertions::assert_eq;

    #[test]
    fn canonicalize_duplicate_types() {
        let reference_type_resolver = ReferenceTypeResolver::new(&Module::dummy());
        let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());

        assert_eq!(
            TypeCanonicalizer::new(reference_type_resolver, type_equality_checker).canonicalize(
                &types::Union::new(
                    vec![
                        types::None::new(SourceInformation::dummy()).into(),
                        types::None::new(SourceInformation::dummy()).into()
                    ],
                    SourceInformation::dummy()
                )
                .into()
            ),
            Ok(types::None::new(SourceInformation::dummy()).into())
        );
    }

    #[test]
    fn canonicalize_nested_union_types() {
        let reference_type_resolver = ReferenceTypeResolver::new(&Module::dummy());
        let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());

        assert_eq!(
            TypeCanonicalizer::new(reference_type_resolver, type_equality_checker).canonicalize(
                &types::Union::new(
                    vec![
                        types::Union::new(
                            vec![
                                types::None::new(SourceInformation::dummy()).into(),
                                types::None::new(SourceInformation::dummy()).into()
                            ],
                            SourceInformation::dummy()
                        )
                        .into(),
                        types::None::new(SourceInformation::dummy()).into()
                    ],
                    SourceInformation::dummy()
                )
                .into()
            ),
            Ok(types::None::new(SourceInformation::dummy()).into())
        );
    }

    #[test]
    fn canonicalize_deeply_nested_union_types() {
        let reference_type_resolver = ReferenceTypeResolver::new(&Module::dummy());
        let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());

        assert_eq!(
            TypeCanonicalizer::new(reference_type_resolver, type_equality_checker).canonicalize(
                &types::Union::new(
                    vec![
                        types::Union::new(
                            vec![
                                types::Union::new(
                                    vec![
                                        types::None::new(SourceInformation::dummy()).into(),
                                        types::None::new(SourceInformation::dummy()).into()
                                    ],
                                    SourceInformation::dummy()
                                )
                                .into(),
                                types::None::new(SourceInformation::dummy()).into()
                            ],
                            SourceInformation::dummy()
                        )
                        .into(),
                        types::None::new(SourceInformation::dummy()).into()
                    ],
                    SourceInformation::dummy()
                )
                .into()
            ),
            Ok(types::None::new(SourceInformation::dummy()).into())
        );
    }

    #[test]
    fn canonicalize_union_types_in_record_type() {
        let reference_type_resolver = ReferenceTypeResolver::new(&Module::dummy());
        let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());

        assert_eq!(
            TypeCanonicalizer::new(reference_type_resolver, type_equality_checker)
                .canonicalize(
                    &types::Record::new(
                        "Foo",
                        vec![(
                            "foo".into(),
                            types::Union::new(
                                vec![
                                    types::None::new(SourceInformation::dummy()).into(),
                                    types::None::new(SourceInformation::dummy()).into()
                                ],
                                SourceInformation::dummy()
                            )
                            .into(),
                        )]
                        .into_iter()
                        .collect(),
                        SourceInformation::dummy()
                    )
                    .into()
                )
                .unwrap()
                .to_record()
                .unwrap()
                .elements(),
            &vec![(
                "foo".into(),
                types::None::new(SourceInformation::dummy()).into(),
            )]
            .into_iter()
            .collect(),
        );
    }

    #[test]
    fn canonicalize_union_types_including_any_types() {
        let reference_type_resolver = ReferenceTypeResolver::new(&Module::dummy());
        let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());

        assert_eq!(
            TypeCanonicalizer::new(reference_type_resolver, type_equality_checker).canonicalize(
                &types::Union::new(
                    vec![
                        types::Any::new(SourceInformation::dummy()).into(),
                        types::None::new(SourceInformation::dummy()).into()
                    ],
                    SourceInformation::dummy()
                )
                .into()
            ),
            Ok(types::Any::new(SourceInformation::dummy()).into())
        );
    }

    #[test]
    fn canonicalize_union_types_including_any_types_in_record_types() {
        let reference_type_resolver = ReferenceTypeResolver::new(&Module::dummy());
        let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());

        assert_eq!(
            TypeCanonicalizer::new(reference_type_resolver, type_equality_checker).canonicalize(
                &types::Record::new(
                    "Foo",
                    vec![(
                        "foo".into(),
                        types::Union::new(
                            vec![
                                types::Any::new(SourceInformation::dummy()).into(),
                                types::None::new(SourceInformation::dummy()).into()
                            ],
                            SourceInformation::dummy()
                        )
                        .into()
                    )]
                    .into_iter()
                    .collect(),
                    SourceInformation::dummy()
                )
                .into()
            ),
            Ok(types::Record::new(
                "Foo",
                vec![(
                    "foo".into(),
                    types::Any::new(SourceInformation::dummy()).into(),
                )]
                .into_iter()
                .collect(),
                SourceInformation::dummy()
            )
            .into())
        );
    }

    #[test]
    fn do_not_canonicalize_normal_union_types() {
        let union_type = types::Union::new(
            vec![
                types::Boolean::new(SourceInformation::dummy()).into(),
                types::None::new(SourceInformation::dummy()).into(),
            ],
            SourceInformation::dummy(),
        );
        let reference_type_resolver = ReferenceTypeResolver::new(&Module::dummy());
        let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());

        assert_eq!(
            TypeCanonicalizer::new(reference_type_resolver, type_equality_checker)
                .canonicalize(&union_type.clone().into()),
            Ok(union_type.into())
        );
    }

    #[test]
    fn canonicalize_record_types_in_union_types() {
        let record_type = types::Record::new("Foo", Default::default(), SourceInformation::dummy());
        let union_type = types::Union::new(
            vec![record_type.clone().into(), record_type.clone().into()],
            SourceInformation::dummy(),
        );
        let reference_type_resolver = ReferenceTypeResolver::new(&Module::dummy());
        let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());

        assert_eq!(
            TypeCanonicalizer::new(reference_type_resolver, type_equality_checker)
                .canonicalize(&union_type.into()),
            Ok(record_type.into())
        );
    }

    #[test]
    fn canonicalize_union_type_resolving_reference_types() {
        let reference_type_resolver =
            ReferenceTypeResolver::new(&Module::from_definitions_and_type_definitions(
                vec![TypeDefinition::new(
                    "Foo",
                    types::Boolean::new(SourceInformation::dummy()),
                )],
                vec![],
            ));
        let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());

        assert_eq!(
            TypeCanonicalizer::new(reference_type_resolver, type_equality_checker).canonicalize(
                &types::Union::new(
                    vec![
                        types::Reference::new("Foo", SourceInformation::dummy()).into(),
                        types::Boolean::new(SourceInformation::dummy()).into(),
                        types::None::new(SourceInformation::dummy()).into(),
                    ],
                    SourceInformation::dummy(),
                )
                .into()
            ),
            Ok(types::Union::new(
                vec![
                    types::Boolean::new(SourceInformation::dummy()).into(),
                    types::None::new(SourceInformation::dummy()).into(),
                ],
                SourceInformation::dummy(),
            )
            .into())
        );
    }

    #[test]
    fn canonicalize_union_type_not_resolving_reference_types() {
        let reference_type_resolver =
            ReferenceTypeResolver::new(&Module::from_definitions_and_type_definitions(
                vec![TypeDefinition::new(
                    "Foo",
                    types::Boolean::new(SourceInformation::dummy()),
                )],
                vec![],
            ));
        let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());

        assert_eq!(
            TypeCanonicalizer::new(reference_type_resolver, type_equality_checker).canonicalize(
                &types::Union::new(
                    vec![
                        types::Reference::new("Foo", SourceInformation::dummy()).into(),
                        types::None::new(SourceInformation::dummy()).into(),
                    ],
                    SourceInformation::dummy(),
                )
                .into()
            ),
            Ok(types::Union::new(
                vec![
                    types::Reference::new("Foo", SourceInformation::dummy()).into(),
                    types::None::new(SourceInformation::dummy()).into(),
                ],
                SourceInformation::dummy(),
            )
            .into())
        );
    }
}
