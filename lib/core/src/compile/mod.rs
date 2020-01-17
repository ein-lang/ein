mod error;
mod expression_compiler;
mod function_compiler;
mod initializer_configuration;
mod initializer_sorter;
mod module;
mod module_compiler;
mod type_compiler;

use super::verify::verify;
use crate::ast;
pub use error::CompileError;
pub use initializer_configuration::InitializerConfiguration;
pub use module::Module;
use module_compiler::ModuleCompiler;
use type_compiler::TypeCompiler;

pub fn compile(
    ast_module: &ast::Module,
    initializer_configuration: &InitializerConfiguration,
) -> Result<Module, CompileError> {
    let ast_module = ast_module.canonicalize_types();

    verify(&ast_module)?;

    let context = llvm::Context::new();
    let module = context.create_module("main");

    ModuleCompiler::new(
        &context,
        &module,
        &ast_module,
        &TypeCompiler::new(&context, &module),
        initializer_configuration,
    )
    .compile()?;

    Ok(Module::new(module))
}
