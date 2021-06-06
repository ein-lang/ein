use super::super::{error::CompileError, reference_type_resolver::ReferenceTypeResolver};
use crate::{
    compile::{type_canonicalizer::TypeCanonicalizer, type_equality_checker::TypeEqualityChecker},
    types::{self, Type},
};
use std::sync::Arc;

pub struct IntersectionTypeCalculator {
    type_canonicalizer: Arc<TypeCanonicalizer>,
    type_equality_checker: Arc<TypeEqualityChecker>,
    reference_type_resolver: Arc<ReferenceTypeResolver>,
}

impl IntersectionTypeCalculator {
    pub fn new(
        type_canonicalizer: Arc<TypeCanonicalizer>,
        type_equality_checker: Arc<TypeEqualityChecker>,
        reference_type_resolver: Arc<ReferenceTypeResolver>,
    ) -> Arc<Self> {
        Self {
            type_canonicalizer,
            type_equality_checker,
            reference_type_resolver,
        }
        .into()
    }

    pub fn calculate(&self, types: &[Type]) -> Result<Type, CompileError> {
        Ok(match types {
            [] => unreachable!(),
            [type_] => type_.clone(),
            [type_, ..] => self.intersect_types(type_, &self.calculate(&types[1..])?)?,
        })
    }

    fn intersect_types(&self, one: &Type, other: &Type) -> Result<Type, CompileError> {
        Ok(
            match (
                self.reference_type_resolver.resolve(one)?,
                self.reference_type_resolver.resolve(other)?,
            ) {
                (Type::Any(_), _) => other.clone(),
                (Type::Boolean(_), _)
                | (Type::Function(_), _)
                | (Type::List(_), _)
                | (Type::None(_), _)
                | (Type::Number(_), _)
                | (Type::Record(_), _)
                | (Type::String(_), _) => one.clone(),
                (Type::Union(one), Type::Union(other)) => self.type_canonicalizer.canonicalize(
                    &types::Union::new(
                        one.types()
                            .iter()
                            .map(|one| -> Result<_, CompileError> {
                                let flags = other
                                    .types()
                                    .iter()
                                    .map(|other| self.type_equality_checker.equal(one, other))
                                    .collect::<Result<Vec<bool>, _>>()?;

                                Ok(if flags.iter().any(|&ok| ok) {
                                    Some(one.clone())
                                } else {
                                    None
                                })
                            })
                            .collect::<Result<Vec<_>, _>>()?
                            .into_iter()
                            .flatten()
                            .collect(),
                        one.source_information().clone(),
                    )
                    .into(),
                )?,
                (Type::Union(_), _) => self.intersect_types(other, one)?,
                (Type::Reference(_), _) | (Type::Unknown(_), _) | (Type::Variable(_), _) => {
                    unreachable!()
                }
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use crate::debug::*;

    fn intersect_types(one: &Type, other: &Type) -> Result<Type, CompileError> {
        let reference_type_resolver = ReferenceTypeResolver::new(&Module::dummy());
        let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());
        let type_canonicalizer = TypeCanonicalizer::new(
            reference_type_resolver.clone(),
            type_equality_checker.clone(),
        );

        IntersectionTypeCalculator::new(
            type_canonicalizer,
            type_equality_checker,
            reference_type_resolver,
        )
        .intersect_types(one, other)
    }

    #[test]
    fn intersect_boolean_types() {
        let type_ = types::Boolean::new(SourceInformation::dummy()).into();

        assert_eq!(intersect_types(&type_, &type_,), Ok(type_));
    }

    #[test]
    fn intersect_function_types() {
        let type_ = types::Function::new(
            types::Boolean::new(SourceInformation::dummy()),
            types::Boolean::new(SourceInformation::dummy()),
            SourceInformation::dummy(),
        )
        .into();

        assert_eq!(intersect_types(&type_, &type_), Ok(type_));
    }

    #[test]
    fn intersect_union_and_boolean_types() {
        assert_eq!(
            intersect_types(
                &types::Union::new(
                    vec![
                        types::Boolean::new(SourceInformation::dummy()).into(),
                        types::None::new(SourceInformation::dummy()).into()
                    ],
                    SourceInformation::dummy()
                )
                .into(),
                &types::Boolean::new(SourceInformation::dummy()).into(),
            ),
            Ok(types::Boolean::new(SourceInformation::dummy()).into())
        );
    }

    #[test]
    fn intersect_boolean_and_union_types() {
        assert_eq!(
            intersect_types(
                &types::Boolean::new(SourceInformation::dummy()).into(),
                &types::Union::new(
                    vec![
                        types::Boolean::new(SourceInformation::dummy()).into(),
                        types::None::new(SourceInformation::dummy()).into()
                    ],
                    SourceInformation::dummy()
                )
                .into(),
            ),
            Ok(types::Boolean::new(SourceInformation::dummy()).into())
        );
    }

    #[test]
    fn intersect_boolean_and_any_types() {
        assert_eq!(
            intersect_types(
                &types::Boolean::new(SourceInformation::dummy()).into(),
                &types::Any::new(SourceInformation::dummy()).into(),
            ),
            Ok(types::Boolean::new(SourceInformation::dummy()).into())
        );
    }

    #[test]
    fn intersect_any_and_boolean_types() {
        assert_eq!(
            intersect_types(
                &types::Any::new(SourceInformation::dummy()).into(),
                &types::Boolean::new(SourceInformation::dummy()).into(),
            ),
            Ok(types::Boolean::new(SourceInformation::dummy()).into())
        );
    }

    #[test]
    fn intersect_union_types() {
        let type_ = types::Union::new(
            vec![
                types::Boolean::new(SourceInformation::dummy()).into(),
                types::None::new(SourceInformation::dummy()).into(),
            ],
            SourceInformation::dummy(),
        )
        .into();

        assert_eq!(intersect_types(&type_, &type_,), Ok(type_));
    }

    #[test]
    fn intersect_unions_into_boolean_type() {
        assert_eq!(
            intersect_types(
                &types::Union::new(
                    vec![
                        types::Boolean::new(SourceInformation::dummy()).into(),
                        types::Number::new(SourceInformation::dummy()).into(),
                    ],
                    SourceInformation::dummy(),
                )
                .into(),
                &types::Union::new(
                    vec![
                        types::Boolean::new(SourceInformation::dummy()).into(),
                        types::None::new(SourceInformation::dummy()).into(),
                    ],
                    SourceInformation::dummy(),
                )
                .into(),
            ),
            Ok(types::Boolean::new(SourceInformation::dummy()).into())
        );
    }
}
