use crate::ast::*;
use crate::types::Type;
use std::collections::HashMap;

pub struct ModuleEnvironmentCreator {}

impl ModuleEnvironmentCreator {
    pub fn create(module: &Module) -> HashMap<String, Type> {
        let mut variables = HashMap::<String, Type>::new();

        for import in module.imports() {
            for (name, type_) in import.module_interface().variables() {
                variables.insert(name.into(), type_.clone());
            }
        }

        for definition in module.definitions() {
            match definition {
                Definition::FunctionDefinition(function_definition) => {
                    variables.insert(
                        function_definition.name().into(),
                        function_definition.type_().clone(),
                    );
                }
                Definition::ValueDefinition(value_definition) => {
                    variables.insert(
                        value_definition.name().into(),
                        value_definition.type_().clone(),
                    );
                }
            }
        }

        variables
    }
}
