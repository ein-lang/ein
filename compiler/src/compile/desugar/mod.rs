use crate::ast::*;

const MAIN_FUNCTION_NAME: &str = "sloth_main";

pub fn desugar(module: &Module) -> Module {
    let mut definitions = Vec::with_capacity(module.definitions().len());

    for definition in module.definitions() {
        match definition {
            Definition::FunctionDefinition(function_definition) => definitions.push(
                FunctionDefinition::new(
                    convert_function_name(function_definition.name()).into(),
                    function_definition.arguments().to_vec(),
                    function_definition.body().clone(),
                    function_definition.type_().clone(),
                )
                .into(),
            ),
            definition => definitions.push(definition.clone()),
        }
    }

    Module::new(definitions)
}

fn convert_function_name(name: &str) -> &str {
    if name == "main" {
        MAIN_FUNCTION_NAME
    } else {
        name
    }
}
