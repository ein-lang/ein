use super::super::error::CompileError;
use super::super::list_literal_configuration::ListLiteralConfiguration;
use crate::ast::*;
use crate::debug::*;
use std::rc::Rc;
use std::sync::Arc;

pub struct ListLiteralPass {
    configuration: Arc<ListLiteralConfiguration>,
}

/// Desugars list literals into generic list functions and variables.
/// Types are consistent after compiling as all `List a` types are converted
/// into `List Any`.
impl ListLiteralPass {
    pub fn new(configuration: Arc<ListLiteralConfiguration>) -> Self {
        Self { configuration }
    }

    pub fn compile(&mut self, module: &Module) -> Result<Module, CompileError> {
        module.convert_expressions(&mut |expression| -> Result<Expression, CompileError> {
            Ok(self.compile_expression(expression))
        })
    }

    fn compile_expression(&mut self, expression: &Expression) -> Expression {
        if let Expression::List(list) = expression {
            self.compile_list(list.elements(), list.source_information())
        } else {
            expression.clone()
        }
    }

    fn compile_list(
        &self,
        elements: &[ListElement],
        source_information: &Rc<SourceInformation>,
    ) -> Expression {
        let rest_expression = || self.compile_list(&elements[1..], source_information);

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
