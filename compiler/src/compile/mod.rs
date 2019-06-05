mod error;
mod expression_compiler;
mod llvm;
mod module_compiler;
mod type_compiler;

use crate::ast;
use error::CompileError;
use module_compiler::ModuleCompiler;
use std::error::Error;
use std::path::Path;

pub struct CompileOptions {
    pub root_directory: String,
}

const BC_PATH: &str = "sloth.bc";

pub fn compile(ast_module: &ast::Module, options: CompileOptions) -> Result<(), CompileError> {
    unsafe {
        let module = llvm::Module::new("main");
        ModuleCompiler::new(&module, ast_module).compile()?;
        llvm::write_bitcode_to_file(module, BC_PATH);
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
