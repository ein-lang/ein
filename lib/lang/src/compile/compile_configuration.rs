use super::error_type_configuration::ErrorTypeConfiguration;
use super::list_type_configuration::ListTypeConfiguration;
use super::main_module_configuration::MainModuleConfiguration;
use super::string_type_configuration::StringTypeConfiguration;
#[cfg(test)]
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;

#[cfg(test)]
pub static COMPILE_CONFIGURATION: Lazy<Arc<CompileConfiguration>> = Lazy::new(|| {
    CompileConfiguration {
        malloc_function_name: "foo_malloc".into(),
        realloc_function_name: "foo_realloc".into(),
        list_type_configuration: super::list_type_configuration::LIST_TYPE_CONFIGURATION.clone(),
        string_type_configuration: super::string_type_configuration::STRING_TYPE_CONFIGURATION
            .clone(),
        error_type_configuration: super::error_type_configuration::ERROR_TYPE_CONFIGURATION.clone(),
        main_module_configuration: None,
    }
    .into()
});

#[derive(Clone)]
pub struct CompileConfiguration {
    pub malloc_function_name: String,
    pub realloc_function_name: String,
    pub error_type_configuration: Arc<ErrorTypeConfiguration>,
    pub list_type_configuration: Arc<ListTypeConfiguration>,
    pub string_type_configuration: Arc<StringTypeConfiguration>,
    pub main_module_configuration: Option<Arc<MainModuleConfiguration>>,
}

impl CompileConfiguration {
    pub fn qualify(&self, names: &HashMap<String, String>) -> Self {
        let mut configuration = self.clone();

        configuration.error_type_configuration =
            self.error_type_configuration.qualify(names).into();
        configuration.list_type_configuration = self.list_type_configuration.qualify(names).into();
        configuration.string_type_configuration =
            self.string_type_configuration.qualify(names).into();
        configuration.main_module_configuration = self
            .main_module_configuration
            .as_ref()
            .map(|configuration| configuration.qualify(names).into());

        configuration
    }
}
