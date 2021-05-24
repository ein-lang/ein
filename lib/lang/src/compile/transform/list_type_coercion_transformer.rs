use super::super::{
    error::CompileError, list_type_configuration::ListTypeConfiguration,
    name_generator::NameGenerator, reference_type_resolver::ReferenceTypeResolver,
    type_equality_checker::TypeEqualityChecker,
};
use crate::{
    ast::*,
    debug::SourceInformation,
    types::{self, Type},
};
use std::sync::Arc;

pub struct ListTypeCoercionTransformer {
    coerce_function_name_generator: NameGenerator,
    argument_name_generator: NameGenerator,
    type_equality_checker: Arc<TypeEqualityChecker>,
    reference_type_resolver: Arc<ReferenceTypeResolver>,
    list_type_configuration: Arc<ListTypeConfiguration>,
}

impl ListTypeCoercionTransformer {
    pub fn new(
        type_equality_checker: Arc<TypeEqualityChecker>,
        reference_type_resolver: Arc<ReferenceTypeResolver>,
        list_type_configuration: Arc<ListTypeConfiguration>,
    ) -> Arc<Self> {
        Self {
            coerce_function_name_generator: NameGenerator::new("$tc_func_"),
            argument_name_generator: NameGenerator::new("$tc_arg_"),
            type_equality_checker,
            reference_type_resolver,
            list_type_configuration,
        }
        .into()
    }

    pub fn transform(&self, coercion: &TypeCoercion) -> Result<Expression, CompileError> {
        Ok(if self.is_coercion_necessary(coercion.to())? {
            self.transform_element(coercion)?
        } else {
            coercion.argument().clone()
        })
    }

    fn transform_element(&self, coercion: &TypeCoercion) -> Result<Expression, CompileError> {
        let coerce_function_name = self.coerce_function_name_generator.generate();
        let argument_name = self.argument_name_generator.generate();

        let from_list_type = self
            .reference_type_resolver
            .resolve_to_list(coercion.from())?
            .unwrap();
        let to_list_type = self
            .reference_type_resolver
            .resolve_to_list(coercion.to())?
            .unwrap();
        let source_information = coercion.source_information().clone();
        let any_list_type = types::Reference::new(
            &self.list_type_configuration.list_type_name,
            source_information.clone(),
        );

        // Re-tag elements if needed.
        Ok(Let::new(
            vec![FunctionDefinition::new(
                coerce_function_name.clone(),
                vec![argument_name.clone()],
                self.coerce_type(
                    self.coerce_type(
                        Case::with_type(
                            types::Any::new(source_information.clone()),
                            argument_name.clone(),
                            Variable::new(argument_name.clone(), source_information.clone()),
                            vec![Alternative::new(
                                from_list_type.element().clone(),
                                Variable::new(argument_name, source_information.clone()),
                            )],
                            source_information.clone(),
                        ),
                        from_list_type.element(),
                        to_list_type.element(),
                        source_information.clone(),
                    )?,
                    to_list_type.element(),
                    &types::Any::new(source_information.clone()).into(),
                    source_information.clone(),
                )?,
                types::Function::new(
                    types::Any::new(source_information.clone()),
                    types::Any::new(source_information.clone()),
                    source_information.clone(),
                ),
                source_information.clone(),
            )
            .into()],
            Application::with_type(
                types::Function::new(
                    any_list_type.clone(),
                    any_list_type.clone(),
                    source_information.clone(),
                ),
                Application::with_type(
                    types::Function::new(
                        types::Function::new(
                            types::Any::new(source_information.clone()),
                            types::Any::new(source_information.clone()),
                            source_information.clone(),
                        ),
                        types::Function::new(
                            any_list_type.clone(),
                            any_list_type,
                            source_information.clone(),
                        ),
                        source_information.clone(),
                    ),
                    Variable::new(
                        &self.list_type_configuration.map_function_name,
                        source_information.clone(),
                    ),
                    Variable::new(coerce_function_name, source_information.clone()),
                    source_information.clone(),
                ),
                coercion.argument().clone(),
                source_information.clone(),
            ),
            source_information,
        )
        .into())
    }

    fn coerce_type(
        &self,
        argument: impl Into<Expression>,
        from_type: &Type,
        to_type: &Type,
        source_information: Arc<SourceInformation>,
    ) -> Result<Expression, CompileError> {
        Ok(if self.type_equality_checker.equal(from_type, to_type)? {
            argument.into()
        } else {
            TypeCoercion::new(
                argument,
                from_type.clone(),
                to_type.clone(),
                source_information,
            )
            .into()
        })
    }

    fn is_coercion_necessary(&self, to_type: &Type) -> Result<bool, CompileError> {
        let list_type = self
            .reference_type_resolver
            .resolve_to_list(to_type)?
            .unwrap();
        let element_type = list_type.element();

        Ok(self.reference_type_resolver.is_function(element_type)?
            || self.reference_type_resolver.is_list(element_type)?)
    }
}
