mod error;
mod expression_compiler;
mod function_compiler;
mod initializer_configuration;
mod llvm;
mod module;
mod module_compiler;
mod type_compiler;

use super::verify::verify;
use crate::ast;
pub use error::CompileError;
pub use initializer_configuration::*;
pub use module::Module;
use module_compiler::ModuleCompiler;
use type_compiler::TypeCompiler;

pub fn compile(
    ast_module: &ast::Module,
    initializer_configuration: &InitializerConfiguration,
) -> Result<Module, CompileError> {
    verify(&ast_module)?;

    let mut module = llvm::Module::new("main");
    ModuleCompiler::new(
        &mut module,
        ast_module,
        &TypeCompiler::new(),
        initializer_configuration,
    )
    .compile()?;

    Ok(Module::new(module))
}
