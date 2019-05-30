mod error;
mod expression_compiler;
mod utilities;

use crate::ast;
use error::CompileError;
use expression_compiler::ExpressionCompiler;
use llvm_sys::analysis::*;
use llvm_sys::bit_writer::*;
use llvm_sys::core::*;
use std::error::Error;
use std::path::Path;
use utilities::c_string;

pub struct CompileOptions {
    pub root_directory: String,
}

const BC_PATH: &str = "sloth.bc";

pub fn compile(expression: &ast::Expression, options: CompileOptions) -> Result<(), CompileError> {
    unsafe {
        let module = LLVMModuleCreateWithName(c_string("main").as_ptr());

        let mut arguments = vec![];
        let function = LLVMAddFunction(
            module,
            c_string("sloth_main").as_ptr(),
            LLVMFunctionType(LLVMDoubleType(), arguments.as_mut_ptr(), 0, 0),
        );
        LLVMAppendBasicBlock(function, c_string("").as_ptr());

        let builder = LLVMCreateBuilder();
        LLVMPositionBuilderAtEnd(builder, LLVMGetEntryBasicBlock(function));
        LLVMBuildRet(
            builder,
            ExpressionCompiler::new(builder).compile(expression)?,
        );

        LLVMVerifyFunction(function, LLVMVerifierFailureAction::LLVMAbortProcessAction);
        LLVMVerifyModule(
            module,
            LLVMVerifierFailureAction::LLVMAbortProcessAction,
            std::ptr::null_mut(),
        );

        LLVMWriteBitcodeToFile(module, c_string(BC_PATH).as_ptr());
    }

    std::process::Command::new("clang")
        .arg("-O3")
        .arg("-flto")
        .arg("-ldl")
        .arg("-lpthread")
        .arg(BC_PATH)
        .arg(Path::new(&options.root_directory).join("target/release/libruntime.a"))
        .output()
        .map_err(|error| CompileError::new(error.description().into()))?;

    Ok(())
}
