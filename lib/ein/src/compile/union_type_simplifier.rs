use super::error::CompileError;
use super::reference_type_resolver::ReferenceTypeResolver;
use crate::types::{self, Type};
use std::collections::HashSet;
use std::rc::Rc;

pub struct UnionTypeSimplifier {
    reference_type_resolver: Rc<ReferenceTypeResolver>,
}

impl UnionTypeSimplifier {
    pub fn new(reference_type_resolver: Rc<ReferenceTypeResolver>) -> Rc<Self> {
        Self {
            reference_type_resolver,
        }
        .into()
    }

    pub fn simplify(&self, type_: &Type) -> Result<Type, CompileError> {
        if let Type::Union(union) = type_ {
            self.simplify_union(union)
        } else {
            Ok(type_.clone())
        }
    }

    pub fn simplify_union(&self, union: &types::Union) -> Result<Type, CompileError> {
        let types = union
            .types()
            .iter()
            .map(|type_| {
                Ok(match self.reference_type_resolver.resolve(type_)? {
                    Type::Union(union) => union.types().iter().cloned().collect(),
                    type_ => vec![type_],
                })
            })
            .collect::<Result<Vec<_>, CompileError>>()?
            .into_iter()
            .flatten()
            .collect::<HashSet<_>>()
            .drain()
            .collect::<Vec<_>>();

        Ok(match types.len() {
            0 => unreachable!(),
            1 => types[0].clone(),
            _ => types::Union::new(types, union.source_information().clone()).into(),
        })
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

        assert_eq!(
            UnionTypeSimplifier::new(reference_type_resolver).simplify_union(&types::Union::new(
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

        assert_eq!(
            UnionTypeSimplifier::new(reference_type_resolver).simplify_union(&types::Union::new(
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
    fn do_not_simplify_normal_union_types() {
        let union_type = types::Union::new(
            vec![
                types::Boolean::new(SourceInformation::dummy()).into(),
                types::None::new(SourceInformation::dummy()).into(),
            ],
            SourceInformation::dummy(),
        );
        let reference_type_resolver = ReferenceTypeResolver::new(&Module::dummy());

        assert_eq!(
            UnionTypeSimplifier::new(reference_type_resolver).simplify_union(&union_type),
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

        assert_eq!(
            UnionTypeSimplifier::new(reference_type_resolver).simplify_union(&union_type),
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

        assert_eq!(
            UnionTypeSimplifier::new(reference_type_resolver).simplify_union(&types::Union::new(
                vec![
                    types::Reference::new("Foo", SourceInformation::dummy()).into(),
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
