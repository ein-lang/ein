use super::super::error::CompileError;
use super::super::list_literal_configuration::ListLiteralConfiguration;
use crate::ast::*;
use crate::debug::*;
use std::sync::Arc;

pub struct ListLiteralTransformer {
    configuration: Arc<ListLiteralConfiguration>,
}

/// Transforms list literals into generic list functions and variables.
/// Types are consistent after transforming as all `List a` types are converted
/// into `List Any`.
impl ListLiteralTransformer {
    pub fn new(configuration: Arc<ListLiteralConfiguration>) -> Self {
        Self { configuration }
    }

    pub fn transform(&mut self, module: &Module) -> Result<Module, CompileError> {
        module.convert_expressions(&mut |expression| -> Result<Expression, CompileError> {
            Ok(self.transform_expression(expression))
        })
    }

    fn transform_expression(&mut self, expression: &Expression) -> Expression {
        if let Expression::List(list) = expression {
            self.transform_list(list.elements(), list.source_information())
        } else {
            expression.clone()
        }
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
