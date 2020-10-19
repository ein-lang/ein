use super::super::list_type_configuration::ListTypeConfiguration;
use crate::ast::*;
use crate::debug::*;
use std::sync::Arc;

pub struct ListLiteralTransformer {
    configuration: Arc<ListTypeConfiguration>,
}

impl ListLiteralTransformer {
    pub fn new(configuration: Arc<ListTypeConfiguration>) -> Arc<Self> {
        Self { configuration }.into()
    }

    pub fn transform(&self, list: &List) -> Expression {
        self.transform_list(list.elements(), list.source_information())
    }

    fn transform_list(
        &self,
        elements: &[ListElement],
        source_information: &Arc<SourceInformation>,
    ) -> Expression {
        let rest_expression = || self.transform_list(&elements[1..], source_information);

        match elements {
            [] => Variable::new(
                self.configuration.empty_list_variable_name(),
                source_information.clone(),
            )
            .into(),
            [ListElement::Multiple(expression), ..] => Application::new(
                Application::new(
                    Variable::new(
                        self.configuration.concatenate_function_name(),
                        source_information.clone(),
                    ),
                    expression.clone(),
                    source_information.clone(),
                ),
                rest_expression(),
                source_information.clone(),
            )
            .into(),
            [ListElement::Single(expression), ..] => Application::new(
                Application::new(
                    Variable::new(
                        self.configuration.prepend_function_name(),
                        source_information.clone(),
                    ),
                    expression.clone(),
                    source_information.clone(),
                ),
                rest_expression(),
                source_information.clone(),
            )
            .into(),
        }
    }
}
