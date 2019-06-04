use super::error::CompileError;
use super::llvm;
use crate::ast;

pub struct ExpressionCompiler<'a> {
    builder: &'a llvm::Builder,
}

impl<'a> ExpressionCompiler<'a> {
    pub fn new(builder: &'a llvm::Builder) -> Self {
        Self { builder }
    }

    pub fn compile(&self, expression: &ast::Expression) -> Result<llvm::Value, CompileError> {
        unsafe {
            Ok(match expression {
                ast::Expression::Operation(operation) => {
                    let lhs = self.compile(operation.lhs())?;
                    let rhs = self.compile(operation.rhs())?;

                    match operation.operator() {
                        ast::Operator::Add => self.builder.build_fadd(lhs, rhs),
                        ast::Operator::Subtract => self.builder.build_fsub(lhs, rhs),
                        ast::Operator::Multiply => self.builder.build_fmul(lhs, rhs),
                        ast::Operator::Divide => self.builder.build_fdiv(lhs, rhs),
                    }
                }
                ast::Expression::Number(number) => llvm::const_real(llvm::double_type(), *number),
            })
        }
    }
}
