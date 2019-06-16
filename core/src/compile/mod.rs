mod error;
mod expression_compiler;
mod llvm;
mod module_compiler;
mod type_check;
mod type_compiler;

use crate::ast;
use error::CompileError;
use module_compiler::ModuleCompiler;
use std::error::Error;
use type_check::check_types;

pub fn compile(ast_module: &ast::Module, bit_code_path: &str) -> Result<(), CompileError> {
    check_types(&ast_module).map_err(|error| CompileError::new(error.description()))?;

    unsafe {
        let module = llvm::Module::new("main");
        ModuleCompiler::new(&module, ast_module).compile()?;
        llvm::write_bitcode_to_file(module, bit_code_path);
    }

    Ok(())
}
