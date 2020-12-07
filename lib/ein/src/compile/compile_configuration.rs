use super::list_type_configuration::ListTypeConfiguration;
use super::string_type_configuration::StringTypeConfiguration;
use super::system_type_configuration::SystemTypeConfiguration;
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
}
