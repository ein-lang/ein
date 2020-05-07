use super::super::error::CompileError;
use crate::ast::*;

pub struct NotEqualOperationDesugarer {}

impl NotEqualOperationDesugarer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn desugar(&mut self, module: &Module) -> Result<Module, CompileError> {
        module.convert_expressions(&mut |expression| -> Result<Expression, CompileError> {
            Ok(self.desugar_expression(expression))
        })
    }

    fn desugar_expression(&mut self, expression: &Expression) -> Expression {
        if let Expression::Operation(operation) = expression {
            if operation.operator() == Operator::NotEqual {
                let source_information = operation.source_information();

                If::new(
                    Operation::with_type(
                        operation.type_().clone(),
                        Operator::Equal,
                        operation.lhs().clone(),
                        operation.rhs().clone(),
                        source_information.clone(),
                    ),
                    Boolean::new(false, source_information.clone()),
                    Boolean::new(true, source_information.clone()),
                    source_information.clone(),
                )
                .into()
            } else {
                expression.clone()
            }
        } else {
            expression.clone()
        }
    }
}
