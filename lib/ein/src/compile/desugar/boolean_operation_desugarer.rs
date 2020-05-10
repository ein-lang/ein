use super::super::error::CompileError;
use crate::ast::*;

pub struct BooleanOperationDesugarer {}

impl BooleanOperationDesugarer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn desugar(&mut self, module: &Module) -> Result<Module, CompileError> {
        module.convert_expressions(&mut |expression| -> Result<Expression, CompileError> {
            Ok(match expression {
                Expression::Operation(operation) => {
                    let source_information = operation.source_information();

                    match operation.operator() {
                        Operator::And => If::new(
                            operation.lhs().clone(),
                            operation.rhs().clone(),
                            Boolean::new(false, source_information.clone()),
                            source_information.clone(),
                        )
                        .into(),
                        Operator::Or => If::new(
                            operation.lhs().clone(),
                            Boolean::new(true, source_information.clone()),
                            operation.rhs().clone(),
                            source_information.clone(),
                        )
                        .into(),
                        _ => expression.clone(),
                    }
                }
                _ => expression.clone(),
            })
        })
    }
}
