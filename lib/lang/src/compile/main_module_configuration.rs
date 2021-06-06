use std::collections::HashMap;

pub struct MainModuleConfiguration {
    pub source_main_function_name: String,
    pub object_main_function_name: String,
    pub main_function_type_name: String,
}

impl MainModuleConfiguration {
    pub fn qualify(&self, names: &HashMap<String, String>) -> Self {
        Self {
            source_main_function_name: self.source_main_function_name.clone(),
            object_main_function_name: self.object_main_function_name.clone(),
            main_function_type_name: self.qualify_name(&self.main_function_type_name, names),
        }
    }

    fn qualify_name(&self, name: &str, names: &HashMap<String, String>) -> String {
        names.get(name).cloned().unwrap_or_else(|| name.into())
    }
}
