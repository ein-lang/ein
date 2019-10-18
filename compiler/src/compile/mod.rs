mod desugar;
mod error;
mod expression_compiler;
mod free_variable_finder;
mod module_compiler;
mod name_generator;
mod type_compiler;
mod type_inference;

use crate::ast;
use desugar::{desugar_with_types, desugar_without_types};
use error::CompileError;
use module_compiler::ModuleCompiler;
use std::io::Write;
use type_inference::infer_types;

pub fn compile(ast_module: &ast::Module, destination: &str) -> Result<(), CompileError> {
    std::fs::File::create(destination)?.write_all(core::compile::compile(
        &ModuleCompiler::new().compile(&desugar_with_types(&infer_types(
            &desugar_without_types(ast_module),
        )?))?,
    )?)?;

    Ok(())
}
