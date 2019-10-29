mod desugar;
mod error;
mod expression_compiler;
mod free_variable_finder;
mod module_compiler;
mod module_interface_compiler;
mod name_generator;
mod name_qualifier;
mod type_compiler;
mod type_inference;

use crate::ast;
use crate::path::ModulePath;
use desugar::{desugar_with_types, desugar_without_types};
use error::CompileError;
use module_compiler::ModuleCompiler;
use module_interface_compiler::ModuleInterfaceCompiler;
use name_qualifier::NameQualifier;
use type_inference::infer_types;

pub type ModuleObject = core::compile::Module;

pub fn compile(module: &ast::Module) -> Result<(ModuleObject, ast::ModuleInterface), CompileError> {
    let module = desugar_with_types(&infer_types(&desugar_without_types(module))?);
    let name_qualifier = NameQualifier::new(&module);

    Ok((
        core::compile::compile(
            &name_qualifier.qualify_core_module(
                &ModuleCompiler::new().compile(&module, module.imported_modules())?,
            ),
            &core::compile::InitializerConfiguration::new(
                if module
                    .definitions()
                    .iter()
                    .any(|definition| definition.name() == "main")
                {
                    "sloth_init".into()
                } else {
                    convert_path_to_initializer_name(module.path())
                },
                module
                    .imported_modules()
                    .iter()
                    .map(|module_interface| {
                        convert_path_to_initializer_name(module_interface.path())
                    })
                    .collect(),
            ),
        )?,
        name_qualifier.qualify_module_interface(&ModuleInterfaceCompiler::new().compile(&module)),
    ))
}

fn convert_path_to_initializer_name(module_path: &ModulePath) -> String {
    module_path.qualify_name("$init")
}
