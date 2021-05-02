use super::super::{error::CompileError, type_canonicalizer::TypeCanonicalizer};
use crate::types::Type;
use std::{collections::HashMap, sync::Arc};

pub struct VariableSubstitutor {
    type_canonicalizer: Arc<TypeCanonicalizer>,
    substitutions: HashMap<usize, Type>,
}

impl VariableSubstitutor {
    pub fn new(
        type_canonicalizer: Arc<TypeCanonicalizer>,
        substitutions: HashMap<usize, Type>,
    ) -> Arc<Self> {
        Self {
            type_canonicalizer,
            substitutions,
        }
        .into()
    }

    pub fn substitute(&self, type_: &Type) -> Result<Type, CompileError> {
        type_.transform_types(&mut |type_| self.substitute_shallowly(type_))
    }

    fn substitute_shallowly(&self, type_: &Type) -> Result<Type, CompileError> {
        self.type_canonicalizer
            .canonicalize(if let Type::Variable(variable) = type_ {
                self.substitutions.get(&variable.id()).ok_or_else(|| {
                    CompileError::TypeNotInferred(variable.source_information().clone())
                })?
            } else {
                type_
            })
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::super::{
            reference_type_resolver::ReferenceTypeResolver,
            type_equality_checker::TypeEqualityChecker,
        },
        *,
    };
    use crate::{ast::Module, debug::SourceInformation, types};

    #[test]
    fn substitute_variable() {
        let reference_type_resolver = ReferenceTypeResolver::new(&Module::dummy());
        let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());
        let type_canonicalizer =
            TypeCanonicalizer::new(reference_type_resolver, type_equality_checker);

        let variable = types::Variable::new(SourceInformation::dummy());

        let substitutor = VariableSubstitutor::new(
            type_canonicalizer,
            vec![(
                variable.id(),
                types::Number::new(SourceInformation::dummy()).into(),
            )]
            .into_iter()
            .collect(),
        );

        assert_eq!(
            substitutor.substitute(&variable.into()),
            Ok(types::Number::new(SourceInformation::dummy()).into())
        );
    }

    #[test]
    fn substitute_variable_recursively() {
        let reference_type_resolver = ReferenceTypeResolver::new(&Module::dummy());
        let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());
        let type_canonicalizer =
            TypeCanonicalizer::new(reference_type_resolver, type_equality_checker);

        let variable = types::Variable::new(SourceInformation::dummy());

        let substitutor = VariableSubstitutor::new(
            type_canonicalizer,
            vec![(
                variable.id(),
                types::Number::new(SourceInformation::dummy()).into(),
            )]
            .into_iter()
            .collect(),
        );

        assert_eq!(
            substitutor.substitute(&types::List::new(variable, SourceInformation::dummy()).into()),
            Ok(types::List::new(
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy()
            )
            .into())
        );
    }
}
