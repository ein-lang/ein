use super::super::error::CompileError;
use super::super::list_literal_configuration::ListLiteralConfiguration;
use crate::ast::*;
use crate::debug::*;
use std::rc::Rc;
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
            self.desugar_list(list.elements(), list.source_information())
        } else {
            expression.clone()
        }
    }

    fn desugar_list(
        &self,
        elements: &[ListElement],
        source_information: &Rc<SourceInformation>,
    ) -> Expression {
        let rest_expression = || self.desugar_list(&elements[1..], source_information);

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
