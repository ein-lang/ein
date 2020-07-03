use super::list_literal_configuration::ListLiteralConfiguration;
use std::sync::Arc;

pub struct CompileConfiguration {
    source_main_function_name: String,
    object_main_function_name: String,
    object_init_function_name: String,
    list_literal_configuration: Arc<ListLiteralConfiguration>,
}

impl CompileConfiguration {
    pub fn new(
        source_main_function_name: impl Into<String>,
        object_main_function_name: impl Into<String>,
        object_init_function_name: impl Into<String>,
        list_literal_configuration: Arc<ListLiteralConfiguration>,
    ) -> Self {
        Self {
            source_main_function_name: source_main_function_name.into(),
            object_main_function_name: object_main_function_name.into(),
            object_init_function_name: object_init_function_name.into(),
            list_literal_configuration,
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

    pub fn list_literal_configuration(&self) -> Arc<ListLiteralConfiguration> {
        self.list_literal_configuration.clone()
    }
}
