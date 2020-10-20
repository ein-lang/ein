use super::super::error::CompileError;
use crate::ast::*;

pub struct NotEqualOperationTransformer {}

impl NotEqualOperationTransformer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn transform(&self, module: &Module) -> Result<Module, CompileError> {
        module.transform_expressions(&mut |expression| -> Result<Expression, CompileError> {
            Ok(self.transform_expression(expression))
        })
    }

    fn transform_expression(&self, expression: &Expression) -> Expression {
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
