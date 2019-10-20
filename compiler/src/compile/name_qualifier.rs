use crate::ast;
use std::collections::HashMap;

#[derive(Debug)]
pub struct NameQualifier {
    names: HashMap<String, String>,
}

impl NameQualifier {
    pub fn new(module_name: &str, module: &ast::Module) -> Self {
        let mut names = HashMap::new();

        for definition in module.definitions() {
            names.insert(
                definition.name().into(),
                format!("{}.{}", module_name, definition.name()),
            );
        }

        names.insert("main".into(), "sloth_main".into());

        Self { names }
    }

    pub fn qualify_module_interface(
        &self,
        module_interface: &ast::ModuleInterface,
    ) -> ast::ModuleInterface {
        ast::ModuleInterface::new(
            module_interface
                .types()
                .iter()
                .map(|(name, type_)| (self.names[name].clone(), type_.clone()))
                .collect(),
        )
    }

    pub fn qualify_core_module(&self, module: &core::ast::Module) -> core::ast::Module {
        core::ast::Module::new(
            module.declarations().iter().cloned().collect(),
            module
                .definitions()
                .iter()
                .map(|definition| match definition {
                    core::ast::Definition::FunctionDefinition(function_definition) => {
                        core::ast::FunctionDefinition::new(
                            self.names
                                .get(function_definition.name())
                                .cloned()
                                .unwrap_or_else(|| function_definition.name().into()),
                            function_definition.environment().iter().cloned().collect(),
                            function_definition.arguments().iter().cloned().collect(),
                            function_definition.body().rename_variables(&self.names),
                            function_definition.result_type().clone(),
                        )
                        .into()
                    }
                    core::ast::Definition::ValueDefinition(value_definition) => {
                        core::ast::ValueDefinition::new(
                            self.names
                                .get(value_definition.name())
                                .cloned()
                                .unwrap_or_else(|| value_definition.name().into()),
                            value_definition.body().rename_variables(&self.names),
                            value_definition.type_().clone(),
                        )
                        .into()
                    }
                })
                .collect(),
        )
    }
}
