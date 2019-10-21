mod desugar;
mod error;
mod expression_compiler;
mod free_variable_finder;
mod module_compiler;
mod module_interface_blob;
mod module_interface_compiler;
mod name_generator;
mod name_qualifier;
mod type_compiler;
mod type_inference;

use crate::ast;
use desugar::{desugar_with_types, desugar_without_types};
use error::CompileError;
use module_compiler::ModuleCompiler;
use module_interface_blob::ModuleInterfaceBlob;
use module_interface_compiler::ModuleInterfaceCompiler;
use name_qualifier::NameQualifier;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use type_inference::infer_types;

pub fn compile(module: &ast::Module, destination: &str) -> Result<(), CompileError> {
    let module = desugar_with_types(&infer_types(&desugar_without_types(module))?);
    let name_qualifier = NameQualifier::new(&module);

    File::create(destination)?.write_all(
        core::compile::compile(&name_qualifier.qualify_core_module(
            &ModuleCompiler::new().compile(&module, module.imported_modules())?,
        ))?
        .as_bytes(),
    )?;

    File::create(Path::new(destination).with_extension("json"))?.write_all(
        ModuleInterfaceBlob::new(
            &name_qualifier
                .qualify_module_interface(&ModuleInterfaceCompiler::new().compile(&module)),
        )?
        .as_bytes(),
    )?;

    Ok(())
}
