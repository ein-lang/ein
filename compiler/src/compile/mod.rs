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
use std::collections::{HashMap, HashSet};
use std::io::Write;
use type_inference::infer_types;

pub fn compile(
    module_name: &str,
    module: &ast::Module,
    destination: &str,
) -> Result<(), CompileError> {
    std::fs::File::create(destination)?.write_all(core::compile::compile(
        &rename_top_level_variables(
            &ModuleCompiler::new().compile(&desugar_with_types(&infer_types(
                &desugar_without_types(module),
            )?))?,
            module_name,
            module.exported_names(),
        ),
    )?)?;

    Ok(())
}

fn rename_top_level_variables(
    module: &core::ast::Module,
    module_name: &str,
    exported_names: &HashSet<String>,
) -> core::ast::Module {
    let mut names = HashMap::new();

    names.insert("main", "sloth_main".into());

    for exported_name in exported_names {
        names.insert(
            exported_name.as_str(),
            format!("{}.{}", module_name, exported_name),
        );
    }

    core::ast::Module::new(
        module
            .definitions()
            .iter()
            .map(|definition| match definition {
                core::ast::Definition::FunctionDefinition(function_definition) => {
                    core::ast::FunctionDefinition::new(
                        names
                            .get(function_definition.name())
                            .cloned()
                            .unwrap_or_else(|| function_definition.name().into()),
                        function_definition.environment().iter().cloned().collect(),
                        function_definition.arguments().iter().cloned().collect(),
                        function_definition.body().rename_variables(&names),
                        function_definition.result_type().clone(),
                    )
                    .into()
                }
                core::ast::Definition::ValueDefinition(value_definition) => {
                    core::ast::ValueDefinition::new(
                        names
                            .get(value_definition.name())
                            .cloned()
                            .unwrap_or_else(|| value_definition.name().into()),
                        value_definition.body().rename_variables(&names),
                        value_definition.type_().clone(),
                    )
                    .into()
                }
            })
            .collect(),
    )
}
