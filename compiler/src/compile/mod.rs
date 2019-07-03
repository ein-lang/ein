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
use std::io::Write;
use std::path::Path;
use type_inference::infer_types;

pub struct CompileOptions {
    pub root_directory: String,
}

const BC_PATH: &str = "sloth.bc";

pub fn compile(ast_module: &ast::Module, options: CompileOptions) -> Result<(), CompileError> {
    core::compile::compile(
        &ModuleCompiler::new().compile(&infer_types(&desugar(ast_module))?)?,
        BC_PATH,
    )?;

    let output = std::process::Command::new("clang")
        .arg("-O3")
        .arg("-flto")
        .arg("-ldl")
        .arg("-lpthread")
        .arg(BC_PATH)
        .arg(Path::new(&options.root_directory).join("target/release/libruntime.a"))
        .output()?;

    if !output.status.success() {
        std::io::stderr().write(&output.stdout)?;
        std::io::stderr().write(&output.stderr)?;

        std::process::exit(output.status.code().unwrap_or(1));
    }

    Ok(())
}
