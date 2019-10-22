mod desugar;
mod error;
mod expression_compiler;
mod free_variable_finder;
mod module_compiler;
mod module_interface_blob;
mod module_interface_compiler;
mod module_object_blob;
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
use module_object_blob::ModuleObjectBlob;
use name_qualifier::NameQualifier;
use type_inference::infer_types;

pub fn compile(
    module: &ast::Module,
) -> Result<(ModuleObjectBlob, ModuleInterfaceBlob), CompileError> {
    let module = desugar_with_types(&infer_types(&desugar_without_types(module))?);
    let name_qualifier = NameQualifier::new(&module);

    Ok((
        ModuleObjectBlob::new(core::compile::compile(
            &name_qualifier.qualify_core_module(
                &ModuleCompiler::new().compile(&module, module.imported_modules())?,
            ),
        )?),
        ModuleInterfaceBlob::new(
            &name_qualifier
                .qualify_module_interface(&ModuleInterfaceCompiler::new().compile(&module)),
        )?,
    ))
}
