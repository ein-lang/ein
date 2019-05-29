use super::error::CompileError;
use crate::ast;
use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::ffi::CString;

pub struct ExpressionCompiler {
    builder: LLVMBuilderRef,
}

impl ExpressionCompiler {
    pub fn new(builder: LLVMBuilderRef) -> ExpressionCompiler {
        ExpressionCompiler { builder }
    }

    pub fn compile(&self, expression: &ast::Expression) -> Result<LLVMValueRef, CompileError> {
        unsafe {
            Ok(match expression {
                ast::Expression::Application(application) => (match application.operator() {
                    ast::Operator::Add => LLVMBuildFAdd,
                    ast::Operator::Subtract => LLVMBuildFSub,
                    ast::Operator::Multiply => LLVMBuildFMul,
                    ast::Operator::Divide => LLVMBuildFDiv,
                })(
                    self.builder,
                    self.compile(application.lhs())?,
                    self.compile(application.rhs())?,
                    CString::new("").unwrap().as_ptr(),
                ),
                ast::Expression::Number(number) => LLVMConstReal(LLVMDoubleType(), *number),
            })
        }
    }
}
