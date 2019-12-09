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

const SOURCE_MAIN_FUNCTION_NAME: &str = "main";
const OBJECT_MAIN_FUNCTION_NAME: &str = "ein_main";
const OBJECT_INIT_FUNCTION_NAME: &str = "ein_init";

pub type ModuleObject = core::compile::Module;

pub fn compile(module: &ast::Module) -> Result<(ModuleObject, ast::ModuleInterface), CompileError> {
    let module = desugar_with_types(&infer_types(&desugar_without_types(module))?);
    let name_qualifier = NameQualifier::new(
        &module,
        vec![(
            SOURCE_MAIN_FUNCTION_NAME.into(),
            OBJECT_MAIN_FUNCTION_NAME.into(),
        )]
        .into_iter()
        .collect(),
    );

    Ok((
        core::compile::compile(
            &name_qualifier.qualify_core_module(&ModuleCompiler::new().compile(&module)?),
            &core::compile::InitializerConfiguration::new(
                if module
                    .definitions()
                    .iter()
                    .any(|definition| definition.name() == SOURCE_MAIN_FUNCTION_NAME)
                {
                    OBJECT_INIT_FUNCTION_NAME.into()
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
        ModuleInterfaceCompiler::new().compile(&module)?,
    ))
}

fn convert_path_to_initializer_name(module_path: &ModulePath) -> String {
    module_path.fully_qualify_name("$init")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use crate::debug::*;
    use crate::types;

    #[test]
    fn compile_constant_initialized_with_operation() {
        assert!(compile(&Module::from_definitions(vec![
            ValueDefinition::new(
                "x",
                Number::new(42.0, SourceInformation::dummy()),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into(),
            ValueDefinition::new(
                "y",
                Operation::new(
                    Operator::Add,
                    Variable::new("x", SourceInformation::dummy()),
                    Number::new(42.0, SourceInformation::dummy()),
                    SourceInformation::dummy()
                ),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()
        ]))
        .is_ok());
    }
}
