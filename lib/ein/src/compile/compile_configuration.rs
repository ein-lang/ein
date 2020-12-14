use super::builtin_configuration::BuiltinConfiguration;
use super::list_type_configuration::ListTypeConfiguration;
use super::string_type_configuration::StringTypeConfiguration;
use super::system_type_configuration::SystemTypeConfiguration;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct CompileConfiguration {
    pub source_main_function_name: String,
    pub object_main_function_name: String,
    pub malloc_function_name: String,
    pub panic_function_name: String,
    pub list_type_configuration: Arc<ListTypeConfiguration>,
    pub string_type_configuration: Arc<StringTypeConfiguration>,
    pub system_type_configuration: Arc<SystemTypeConfiguration>,
    pub builtin_configuration: Arc<BuiltinConfiguration>,
}

impl CompileConfiguration {
    pub fn qualify(&self, names: &HashMap<String, String>) -> Self {
        let mut configuration = self.clone();

        configuration.list_type_configuration = self.list_type_configuration.qualify(names).into();
        configuration.system_type_configuration =
            self.system_type_configuration.qualify(names).into();

        configuration
    }
}
