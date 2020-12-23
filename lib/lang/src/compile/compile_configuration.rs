use super::builtin_configuration::BuiltinConfiguration;
use super::error_type_configuration::ErrorTypeConfiguration;
use super::list_type_configuration::ListTypeConfiguration;
use super::string_type_configuration::StringTypeConfiguration;
use super::system_type_configuration::SystemTypeConfiguration;
#[cfg(test)]
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Arc;

#[cfg(test)]
lazy_static! {
    pub static ref COMPILE_CONFIGURATION: Arc<CompileConfiguration> = CompileConfiguration {
        source_main_function_name: "main".into(),
        object_main_function_name: "foo_main".into(),
        malloc_function_name: "foo_malloc".into(),
        panic_function_name: "foo_panic".into(),
        list_type_configuration: super::list_type_configuration::LIST_TYPE_CONFIGURATION.clone(),
        string_type_configuration: super::string_type_configuration::STRING_TYPE_CONFIGURATION
            .clone(),
        system_type_configuration: super::system_type_configuration::SYSTEM_TYPE_CONFIGURATION
            .clone(),
        error_type_configuration: super::error_type_configuration::ERROR_TYPE_CONFIGURATION.clone(),
        builtin_configuration: super::builtin_configuration::BUILTIN_CONFIGURATION.clone(),
    }
    .into();
}

#[derive(Clone)]
pub struct CompileConfiguration {
    pub source_main_function_name: String,
    pub object_main_function_name: String,
    pub malloc_function_name: String,
    pub panic_function_name: String,
    pub error_type_configuration: Arc<ErrorTypeConfiguration>,
    pub list_type_configuration: Arc<ListTypeConfiguration>,
    pub string_type_configuration: Arc<StringTypeConfiguration>,
    pub system_type_configuration: Arc<SystemTypeConfiguration>,
    pub builtin_configuration: Arc<BuiltinConfiguration>,
}

impl CompileConfiguration {
    pub fn qualify(&self, names: &HashMap<String, String>) -> Self {
        let mut configuration = self.clone();

        configuration.error_type_configuration =
            self.error_type_configuration.qualify(names).into();
        configuration.list_type_configuration = self.list_type_configuration.qualify(names).into();
        configuration.system_type_configuration =
            self.system_type_configuration.qualify(names).into();

        configuration
    }
}
