use crate::ast::*;

const MAIN_FUNCTION_NAME: &str = "sloth_main";

pub fn desugar(module: &Module) -> Module {
    Module::new(
        module
            .definitions()
            .iter()
            .map(|definition| match definition {
                Definition::FunctionDefinition(function_definition) => FunctionDefinition::new(
                    convert_function_name(function_definition.name()),
                    function_definition.arguments().to_vec(),
                    function_definition.body().clone(),
                    function_definition.type_().clone(),
                    function_definition.source_information().clone(),
                )
                .into(),
                definition => definition.clone(),
            })
            .collect(),
    )
}

fn convert_function_name(name: &str) -> &str {
    if name == "main" {
        MAIN_FUNCTION_NAME
    } else {
        name
    }
}
