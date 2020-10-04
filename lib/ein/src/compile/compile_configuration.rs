use super::list_type_configuration::ListTypeConfiguration;
use std::sync::Arc;

pub struct CompileConfiguration {
    source_main_function_name: String,
    object_main_function_name: String,
    object_init_function_name: String,
    malloc_function_name: String,
    panic_function_name: String,
    list_type_configuration: Arc<ListTypeConfiguration>,
}

impl CompileConfiguration {
    pub fn new(
        source_main_function_name: impl Into<String>,
        object_main_function_name: impl Into<String>,
        object_init_function_name: impl Into<String>,
        malloc_function_name: impl Into<String>,
        panic_function_name: impl Into<String>,
        list_type_configuration: Arc<ListTypeConfiguration>,
    ) -> Self {
        Self {
            source_main_function_name: source_main_function_name.into(),
            object_main_function_name: object_main_function_name.into(),
            object_init_function_name: object_init_function_name.into(),
            malloc_function_name: malloc_function_name.into(),
            panic_function_name: panic_function_name.into(),
            list_type_configuration,
        }
    }

    pub fn source_main_function_name(&self) -> &str {
        &self.source_main_function_name
    }

    pub fn object_main_function_name(&self) -> &str {
        &self.object_main_function_name
    }

    pub fn object_init_function_name(&self) -> &str {
        &self.object_init_function_name
    }

    pub fn malloc_function_name(&self) -> &str {
        &self.malloc_function_name
    }

    pub fn panic_function_name(&self) -> &str {
        &self.panic_function_name
    }

    pub fn list_type_configuration(&self) -> Arc<ListTypeConfiguration> {
        self.list_type_configuration.clone()
    }
}
