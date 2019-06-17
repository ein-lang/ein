mod desugar;
mod error;
mod expression_compiler;
mod module_compiler;
mod type_compiler;
mod type_inference;

use crate::ast;
use desugar::desugar;
use error::CompileError;
use module_compiler::ModuleCompiler;
use std::error::Error;
use std::path::Path;
use type_inference::infer_types;

pub struct CompileOptions {
    pub root_directory: String,
}

const BC_PATH: &str = "sloth.bc";

pub fn compile(ast_module: &ast::Module, options: CompileOptions) -> Result<(), CompileError> {
    core::compile::compile(
        &ModuleCompiler::new().compile(
            &infer_types(&desugar(ast_module))
                .map_err(|error| CompileError::new(error.description()))?,
        )?,
        BC_PATH,
    )
    .map_err(|error| CompileError::new(error.description()))?;

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
