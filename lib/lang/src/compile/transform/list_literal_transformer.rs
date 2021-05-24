use super::super::{
    error::CompileError, list_type_configuration::ListTypeConfiguration,
    reference_type_resolver::ReferenceTypeResolver,
};
use crate::{ast::*, debug::*, types};
use std::sync::Arc;

pub struct ListLiteralTransformer {
    reference_type_resolver: Arc<ReferenceTypeResolver>,
    configuration: Arc<ListTypeConfiguration>,
}

impl ListLiteralTransformer {
    pub fn new(
        reference_type_resolver: Arc<ReferenceTypeResolver>,
        configuration: Arc<ListTypeConfiguration>,
    ) -> Arc<Self> {
        Self {
            reference_type_resolver,
            configuration,
        }
        .into()
    }

    pub fn transform(&self, list: &List) -> Result<Expression, CompileError> {
        Ok(self.transform_list(
            &self
                .reference_type_resolver
                .resolve_to_list(list.type_())?
                .unwrap(),
            list.elements(),
            list.source_information(),
        ))
    }

    fn transform_list(
        &self,
        type_: &types::List,
        elements: &[ListElement],
        source_information: &Arc<SourceInformation>,
    ) -> Expression {
        let rest_expression = || self.transform_list(type_, &elements[1..], source_information);
        let list_type = types::Reference::new(
            self.configuration.list_type_name.clone(),
            source_information.clone(),
        );

        match elements {
            [] => Variable::new(
                &self.configuration.empty_list_variable_name,
                source_information.clone(),
            )
            .into(),
            [ListElement::Multiple(expression), ..] => Application::with_type(
                types::Function::new(
                    list_type.clone(),
                    list_type.clone(),
                    source_information.clone(),
                ),
                Application::with_type(
                    types::Function::new(
                        list_type.clone(),
                        types::Function::new(
                            list_type.clone(),
                            list_type,
                            source_information.clone(),
                        ),
                        source_information.clone(),
                    ),
                    Variable::new(
                        &self.configuration.concatenate_function_name,
                        source_information.clone(),
                    ),
                    expression.clone(),
                    source_information.clone(),
                ),
                rest_expression(),
                source_information.clone(),
            )
            .into(),
            [ListElement::Single(expression), ..] => Application::with_type(
                types::Function::new(
                    list_type.clone(),
                    list_type.clone(),
                    source_information.clone(),
                ),
                Application::with_type(
                    types::Function::new(
                        types::Any::new(source_information.clone()),
                        types::Function::new(
                            list_type.clone(),
                            list_type,
                            source_information.clone(),
                        ),
                        source_information.clone(),
                    ),
                    Variable::new(
                        &self.configuration.prepend_function_name,
                        source_information.clone(),
                    ),
                    TypeCoercion::new(
                        expression.clone(),
                        type_.element().clone(),
                        types::Any::new(source_information.clone()),
                        source_information.clone(),
                    ),
                    source_information.clone(),
                ),
                rest_expression(),
                source_information.clone(),
            )
            .into(),
        }
    }
}
