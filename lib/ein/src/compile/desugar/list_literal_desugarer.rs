use super::super::error::CompileError;
use super::super::list_literal_configuration::ListLiteralConfiguration;
use crate::ast::*;
use std::sync::Arc;

pub struct ListLiteralDesugarer {
    configuration: Arc<ListLiteralConfiguration>,
}

/// Desugars list literals into generic list functions and variables.
/// Types are consistent after desugaring as all `List a` types are converted
/// into `List Any`.
impl ListLiteralDesugarer {
    pub fn new(configuration: Arc<ListLiteralConfiguration>) -> Self {
        Self { configuration }
    }

    pub fn desugar(&mut self, module: &Module) -> Result<Module, CompileError> {
        module.convert_expressions(&mut |expression| -> Result<Expression, CompileError> {
            Ok(self.desugar_expression(expression))
        })
    }

    fn desugar_expression(&mut self, expression: &Expression) -> Expression {
        if let Expression::List(list) = expression {
            self.desugar_list(list)
        } else {
            expression.clone()
        }
    }

    fn desugar_list(&self, list: &List) -> Expression {
        let source_information = list.source_information();

        match list.elements() {
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
                self.desugar_rest(list),
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
                self.desugar_rest(list),
                source_information.clone(),
            )
            .into(),
        }
    }

    fn desugar_rest(&self, list: &List) -> Expression {
        self.desugar_list(&List::new(
            list.elements()[1..].to_vec(),
            list.source_information().clone(),
        ))
    }
}
