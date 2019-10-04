mod desugar;
mod error;
mod expression_compiler;
mod free_variable_finder;
mod module_compiler;
mod type_compiler;
mod type_inference;

use crate::ast;
use desugar::desugar;
use error::CompileError;
use module_compiler::ModuleCompiler;
use type_inference::infer_types;

pub fn compile(ast_module: &ast::Module, destination: &str) -> Result<(), CompileError> {
    core::compile::compile(
        &ModuleCompiler::new().compile(&infer_types(&desugar(ast_module))?)?,
        destination,
    )?;

    Ok(())
}
