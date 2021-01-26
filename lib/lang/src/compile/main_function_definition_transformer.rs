use super::error::CompileError;
use super::main_module_configuration::MainModuleConfiguration;
use crate::ast::*;
use crate::types;
use std::collections::HashMap;
use std::sync::Arc;

const ARGUMENT_NAME: &str = "$argument";

pub struct MainFunctionDefinitionTransformer {
    global_names: Arc<HashMap<String, String>>,
    main_module_configuration: Arc<MainModuleConfiguration>,
}

impl MainFunctionDefinitionTransformer {
    pub fn new(
        global_names: Arc<HashMap<String, String>>,
        main_module_configuration: Arc<MainModuleConfiguration>,
    ) -> Self {
        Self {
            global_names,
            main_module_configuration,
        }
    }

    pub fn transform(&self, module: &Module) -> Result<Module, CompileError> {
        let main_function_name = self
            .global_names
            .get(&self.main_module_configuration.source_main_function_name)
            .ok_or_else(|| CompileError::MainFunctionNotFound(module.path().clone()))?;
        let source_information = module
            .definitions()
            .iter()
            .find(|definition| definition.name() == main_function_name)
            .ok_or_else(|| CompileError::MainFunctionNotFound(module.path().clone()))?
            .source_information();

        Ok(Module::new(
            module.path().clone(),
            module.export().clone(),
            module.export_foreign().clone(),
            module.imports().to_vec(),
            module.import_foreigns().to_vec(),
            module.type_definitions().to_vec(),
            module
                .definitions()
                .iter()
                .cloned()
                .chain(vec![FunctionDefinition::new(
                    &self.main_module_configuration.object_main_function_name,
                    vec![ARGUMENT_NAME.into()],
                    Application::new(
                        Variable::new(main_function_name, source_information.clone()),
                        Variable::new(ARGUMENT_NAME, source_information.clone()),
                        source_information.clone(),
                    ),
                    types::Function::new(
                        types::Reference::new(
                            &self.main_module_configuration.argument_type_name,
                            source_information.clone(),
                        ),
                        types::Number::new(source_information.clone()),
                        source_information.clone(),
                    ),
                    source_information.clone(),
                )
                .into()])
                .collect(),
        ))
    }
}
