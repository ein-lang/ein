mod error;
mod expression_compiler;
mod function_compiler;
mod llvm;
mod module;
mod module_compiler;
mod object_blob;
mod type_compiler;

use super::verify::verify;
use crate::ast;
pub use error::CompileError;
pub use module::Module;
use module_compiler::ModuleCompiler;
pub use object_blob::ObjectBlob;
use type_compiler::TypeCompiler;

pub fn compile(ast_module: &ast::Module) -> Result<Module, CompileError> {
    verify(&ast_module)?;

    let module = llvm::Module::new("main");
    ModuleCompiler::new(module, ast_module, &TypeCompiler::new()).compile()?;

    Ok(Module::new(module))
}
