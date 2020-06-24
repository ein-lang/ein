use super::error::CompileError;
use super::reference_type_resolver::ReferenceTypeResolver;
use super::type_equality_checker::TypeEqualityChecker;
use crate::types::{self, Type};
use std::rc::Rc;

pub struct UnionTypeSimplifier {
    reference_type_resolver: Rc<ReferenceTypeResolver>,
    type_equality_checker: Rc<TypeEqualityChecker>,
}

impl UnionTypeSimplifier {
    pub fn new(
        reference_type_resolver: Rc<ReferenceTypeResolver>,
        type_equality_checker: Rc<TypeEqualityChecker>,
    ) -> Rc<Self> {
        Self {
            reference_type_resolver,
            type_equality_checker,
        }
        .into()
    }

    pub fn simplify(&self, type_: &Type) -> Result<Type, CompileError> {
        Ok(if let Type::Union(union) = type_ {
            self.simplify_union(&union)?
        } else {
            type_.clone()
        })
    }

    // This function assumes that contained types are canonicalized already.
    pub fn simplify_union(&self, union: &types::Union) -> Result<Type, CompileError> {
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
                    type_ => vec![type_],
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
    fn simplify_duplicate_types() {
        let reference_type_resolver = ReferenceTypeResolver::new(&Module::dummy());
        let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());

        assert_eq!(
            UnionTypeSimplifier::new(reference_type_resolver, type_equality_checker)
                .simplify_union(&types::Union::new(
                    vec![
                        types::None::new(SourceInformation::dummy()).into(),
                        types::None::new(SourceInformation::dummy()).into()
                    ],
                    SourceInformation::dummy()
                )),
            Ok(types::None::new(SourceInformation::dummy()).into())
        );
    }

    #[test]
    fn simplify_nested_union_types() {
        let reference_type_resolver = ReferenceTypeResolver::new(&Module::dummy());
        let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());

        assert_eq!(
            UnionTypeSimplifier::new(reference_type_resolver, type_equality_checker)
                .simplify_union(&types::Union::new(
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
                )),
            Ok(types::None::new(SourceInformation::dummy()).into())
        );
    }

    #[test]
    fn simplify_deeply_nested_union_types() {
        let reference_type_resolver = ReferenceTypeResolver::new(&Module::dummy());
        let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());

        assert_eq!(
            UnionTypeSimplifier::new(reference_type_resolver, type_equality_checker)
                .simplify_union(&types::Union::new(
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
                )),
            Ok(types::None::new(SourceInformation::dummy()).into())
        );
    }

    #[test]
    fn simplify_union_types_including_any_types() {
        let reference_type_resolver = ReferenceTypeResolver::new(&Module::dummy());
        let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());

        assert_eq!(
            UnionTypeSimplifier::new(reference_type_resolver, type_equality_checker)
                .simplify_union(&types::Union::new(
                    vec![
                        types::Any::new(SourceInformation::dummy()).into(),
                        types::None::new(SourceInformation::dummy()).into()
                    ],
                    SourceInformation::dummy()
                )),
            Ok(types::Any::new(SourceInformation::dummy()).into())
        );
    }

    #[test]
    fn simplify_union_types_including_any_types_in_record_types() {
        let reference_type_resolver = ReferenceTypeResolver::new(&Module::dummy());
        let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());

        assert_eq!(
            UnionTypeSimplifier::new(reference_type_resolver, type_equality_checker).simplify(
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
    fn do_not_simplify_normal_union_types() {
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
            UnionTypeSimplifier::new(reference_type_resolver, type_equality_checker)
                .simplify_union(&union_type),
            Ok(union_type.into())
        );
    }

    #[test]
    fn simplify_record_types_in_union_types() {
        let record_type = types::Record::new("Foo", Default::default(), SourceInformation::dummy());
        let union_type = types::Union::new(
            vec![record_type.clone().into(), record_type.clone().into()],
            SourceInformation::dummy(),
        );
        let reference_type_resolver = ReferenceTypeResolver::new(&Module::dummy());
        let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());

        assert_eq!(
            UnionTypeSimplifier::new(reference_type_resolver, type_equality_checker)
                .simplify_union(&union_type),
            Ok(record_type.into())
        );
    }

    #[test]
    fn simplify_union_type_resolving_reference_types() {
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
            UnionTypeSimplifier::new(reference_type_resolver, type_equality_checker)
                .simplify_union(&types::Union::new(
                    vec![
                        types::Reference::new("Foo", SourceInformation::dummy()).into(),
                        types::Boolean::new(SourceInformation::dummy()).into(),
                        types::None::new(SourceInformation::dummy()).into(),
                    ],
                    SourceInformation::dummy(),
                )),
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
}
