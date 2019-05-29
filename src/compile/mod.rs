mod error;
mod expression_compiler;
mod utilities;

use crate::ast;
use error::CompileError;
use expression_compiler::ExpressionCompiler;
use llvm_sys::core::*;
use utilities::c_string;

pub fn compile(expression: &ast::Expression) -> Result<(), CompileError> {
    unsafe {
        let module = LLVMModuleCreateWithName(std::mem::transmute(c_string("main")));
        let mut arguments = vec![];
        let function = LLVMAddFunction(
            module,
            c_string("main"),
            LLVMFunctionType(LLVMVoidType(), arguments.as_mut_ptr(), 0, 0),
        );
        let builder = LLVMCreateBuilder();
        LLVMPositionBuilderAtEnd(builder, LLVMGetEntryBasicBlock(function));
        let compiler = ExpressionCompiler::new(builder);
        compiler.compile(expression)?;
    }

    Ok(())
}
