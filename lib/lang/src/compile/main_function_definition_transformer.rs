use super::{error::CompileError, main_module_configuration::MainModuleConfiguration};
use crate::{ast::*, types};
use std::sync::Arc;

pub struct MainFunctionDefinitionTransformer {
    main_module_configuration: Arc<MainModuleConfiguration>,
}

impl MainFunctionDefinitionTransformer {
    pub fn new(main_module_configuration: Arc<MainModuleConfiguration>) -> Self {
        Self {
            main_module_configuration,
        }
    }

    pub fn transform(&self, module: &Module) -> Result<Module, CompileError> {
        let source_information = module
            .definitions()
            .iter()
            .find(|definition| {
                definition.name() == self.main_module_configuration.source_main_function_name
            })
            .ok_or_else(|| CompileError::MainFunctionNotFound(module.path().clone()))?
            .source_information();

        Ok(Module::new(
            module.path().clone(),
            module.export().clone(),
            ExportForeign::new(
                module
                    .export_foreign()
                    .names()
                    .iter()
                    .cloned()
                    .chain(vec![self
                        .main_module_configuration
                        .object_main_function_name
                        .clone()])
                    .collect(),
            ),
            module.imports().to_vec(),
            module.import_foreigns().to_vec(),
            module.type_definitions().to_vec(),
            module
                .definitions()
                .iter()
                .cloned()
                .chain(vec![VariableDefinition::new(
                    &self.main_module_configuration.object_main_function_name,
                    Variable::new(
                        &self.main_module_configuration.source_main_function_name,
                        source_information.clone(),
                    ),
                    types::Reference::new(
                        &self.main_module_configuration.main_function_type_name,
                        source_information.clone(),
                    ),
                    source_information.clone(),
                )
                .into()])
                .collect(),
        ))
    }
}
