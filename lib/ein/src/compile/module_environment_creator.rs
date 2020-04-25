use crate::ast::*;
use crate::types::{self, Type};
use std::collections::HashMap;

pub struct ModuleEnvironmentCreator {}

impl ModuleEnvironmentCreator {
    pub fn create(module: &Module) -> HashMap<String, Type> {
        let mut variables = HashMap::<String, Type>::new();

        for imported_module in module.imported_modules() {
            for (name, type_) in imported_module.variables() {
                variables.insert(imported_module.path().qualify_name(name), type_.clone());
            }
        }

        for type_definition in module.type_definitions() {
            if let Type::Record(record) = type_definition.type_() {
                for (key, type_) in record.elements() {
                    variables.insert(
                        format!("{}.{}", record.name(), key),
                        types::Function::new(
                            record.clone(),
                            type_.clone(),
                            type_.source_information().clone(),
                        )
                        .into(),
                    );
                }
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
