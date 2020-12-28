use super::super::error::CompileError;
use super::super::error_type_configuration::ErrorTypeConfiguration;
use super::super::reference_type_resolver::ReferenceTypeResolver;
use super::super::type_canonicalizer::TypeCanonicalizer;
use super::super::type_equality_checker::TypeEqualityChecker;
use crate::ast::*;
use crate::debug::SourceInformation;
use crate::types::{self, Type};
use std::sync::Arc;

pub struct LetErrorTransformer {
    reference_type_resolver: Arc<ReferenceTypeResolver>,
    type_equality_checker: Arc<TypeEqualityChecker>,
    type_canonicalizer: Arc<TypeCanonicalizer>,
    error_type_configuration: Arc<ErrorTypeConfiguration>,
}

impl LetErrorTransformer {
    pub fn new(
        reference_type_resolver: Arc<ReferenceTypeResolver>,
        type_equality_checker: Arc<TypeEqualityChecker>,
        type_canonicalizer: Arc<TypeCanonicalizer>,
        error_type_configuration: Arc<ErrorTypeConfiguration>,
    ) -> Arc<Self> {
        Self {
            reference_type_resolver,
            type_equality_checker,
            type_canonicalizer,
            error_type_configuration,
        }
        .into()
    }

    pub fn transform(&self, let_: &LetError) -> Result<Expression, CompileError> {
        let error_type = types::Reference::new(
            &self.error_type_configuration.error_type_name,
            let_.source_information().clone(),
        );

        let_.definitions().iter().rev().fold(
            Ok(let_.expression().clone()),
            |expression, variable_definition| {
                let ok_type = variable_definition.type_().clone();

                Ok(Case::with_type(
                    self.type_canonicalizer.canonicalize(
                        &types::Union::new(
                            vec![ok_type.clone(), error_type.clone().into()],
                            let_.source_information().clone(),
                        )
                        .into(),
                    )?,
                    variable_definition.name(),
                    variable_definition.body().clone(),
                    vec![
                        Alternative::new(ok_type, expression?),
                        Alternative::new(
                            error_type.clone(),
                            self.coerce_type(
                                &Variable::new(
                                    variable_definition.name(),
                                    let_.source_information().clone(),
                                )
                                .into(),
                                &error_type.clone().into(),
                                &self.type_canonicalizer.canonicalize(
                                    &types::Union::new(
                                        vec![let_.type_().clone(), error_type.clone().into()],
                                        let_.source_information().clone(),
                                    )
                                    .into(),
                                )?,
                                let_.source_information().clone(),
                            )?,
                        ),
                    ],
                    let_.source_information().clone(),
                )
                .into())
            },
        )
    }

    fn coerce_type(
        &self,
        expression: &Expression,
        from_type: &Type,
        to_type: &Type,
        source_information: Arc<SourceInformation>,
    ) -> Result<Expression, CompileError> {
        Ok(
            if self.type_equality_checker.equal(&from_type, &to_type)?
                || self.reference_type_resolver.is_list(&from_type)?
                    && self.reference_type_resolver.is_list(&to_type)?
            {
                expression.clone()
            } else {
                TypeCoercion::new(
                    expression.clone(),
                    from_type.clone(),
                    to_type.clone(),
                    source_information,
                )
                .into()
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::error_type_configuration::ERROR_TYPE_CONFIGURATION;
    use super::*;
    use crate::debug::*;
    use crate::package::Package;
    use crate::path::ModulePath;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref MODULE: Module = Module::new(
            ModulePath::new(Package::new("", ""), vec![]),
            Export::new(Default::default()),
            vec![Import::new(
                ModuleInterface::new(
                    ModulePath::new(Package::new("m", ""), vec![]),
                    Default::default(),
                    vec![(
                        "Error".into(),
                        types::Record::new("Error", Default::default(), SourceInformation::dummy())
                            .into(),
                    )]
                    .into_iter()
                    .collect(),
                    Default::default(),
                ),
                false,
            )],
            vec![],
            vec![],
            vec![],
        );
    }

    fn create_let_error_transformer() -> Arc<LetErrorTransformer> {
        let reference_type_resolver = ReferenceTypeResolver::new(&MODULE);
        let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());
        let type_canonicalizer = TypeCanonicalizer::new(
            reference_type_resolver.clone(),
            type_equality_checker.clone(),
        );

        LetErrorTransformer::new(
            reference_type_resolver,
            type_equality_checker,
            type_canonicalizer,
            ERROR_TYPE_CONFIGURATION.clone(),
        )
    }

    #[test]
    fn transform() {
        let error_type = types::Reference::new("Error", SourceInformation::dummy());

        insta::assert_debug_snapshot!(create_let_error_transformer().transform(
            &LetError::with_type(
                types::Union::new(
                    vec![
                        types::None::new(SourceInformation::dummy()).into(),
                        error_type.into()
                    ],
                    SourceInformation::dummy()
                ),
                vec![VariableDefinition::new(
                    "x",
                    Variable::new("y", SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )],
                None::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
        ));
    }
}
