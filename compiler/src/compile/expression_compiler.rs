use super::error::CompileError;
use super::llvm;
use crate::ast;
use std::collections::HashMap;

pub struct ExpressionCompiler<'a> {
    builder: &'a llvm::Builder,
    variables: &'a HashMap<String, llvm::Value>,
}

impl<'a> ExpressionCompiler<'a> {
    pub fn new(builder: &'a llvm::Builder, variables: &'a HashMap<String, llvm::Value>) -> Self {
        Self { builder, variables }
    }

    pub fn compile(&self, expression: &ast::Expression) -> Result<llvm::Value, CompileError> {
        unsafe {
            match expression {
                ast::Expression::Number(number) => {
                    Ok(llvm::const_real(llvm::double_type(), *number))
                }
                ast::Expression::Operation(operation) => {
                    let lhs = self.compile(operation.lhs())?;
                    let rhs = self.compile(operation.rhs())?;

                    Ok(match operation.operator() {
                        ast::Operator::Add => self.builder.build_fadd(lhs, rhs),
                        ast::Operator::Subtract => self.builder.build_fsub(lhs, rhs),
                        ast::Operator::Multiply => self.builder.build_fmul(lhs, rhs),
                        ast::Operator::Divide => self.builder.build_fdiv(lhs, rhs),
                    })
                }
                ast::Expression::Variable(name) => match self.variables.get(name) {
                    Some(value) => Ok(*value),
                    None => Err(CompileError::new("variable not found")),
                },
            }
        }
    }
}
