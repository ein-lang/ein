use super::builtin_configuration::BuiltinConfiguration;
use crate::ast::*;
use crate::types::Type;
use std::collections::HashMap;
use std::sync::Arc;

pub struct ModuleEnvironmentCreator {
    builtin_configuration: Arc<BuiltinConfiguration>,
}

impl ModuleEnvironmentCreator {
    pub fn new(builtin_configuration: Arc<BuiltinConfiguration>) -> Arc<Self> {
        Self {
            builtin_configuration,
        }
        .into()
    }

    pub fn create(&self, module: &Module) -> HashMap<String, Type> {
        let mut variables = HashMap::<String, Type>::new();

        for (name, type_) in &self.builtin_configuration.functions {
            variables.insert(name.into(), type_.clone().into());
        }

        for import in module.imports() {
            for (name, type_) in import.module_interface().functions() {
                variables.insert(name.into(), type_.clone());
            }

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
                Definition::VariableDefinition(variable_definition) => {
                    variables.insert(
                        variable_definition.name().into(),
                        variable_definition.type_().clone(),
                    );
                }
            }
        }

        variables
    }
}
