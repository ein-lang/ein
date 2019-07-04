mod error;
mod expression_compiler;
mod function_compiler;
mod llvm;
mod module_compiler;
mod type_compiler;

use super::verify::verify;
use crate::ast;
pub use error::CompileError;
use module_compiler::ModuleCompiler;
use type_compiler::TypeCompiler;

pub fn compile(ast_module: &ast::Module, bit_code_path: &str) -> Result<(), CompileError> {
    verify(&ast_module)?;

    let module = llvm::Module::new("main");
    ModuleCompiler::new(module, ast_module, &TypeCompiler::new()).compile()?;
    llvm::write_bitcode_to_file(module, bit_code_path);

    Ok(())
}
