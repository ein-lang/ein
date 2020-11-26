use super::list_type_configuration::ListTypeConfiguration;
use std::sync::Arc;

#[derive(Clone)]
pub struct CompileConfiguration {
    pub source_main_function_name: String,
    pub object_main_function_name: String,
    pub malloc_function_name: String,
    pub panic_function_name: String,
    pub list_type_configuration: Arc<ListTypeConfiguration>,
}
