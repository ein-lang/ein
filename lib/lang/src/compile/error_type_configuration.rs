use std::collections::HashMap;

#[cfg(test)]
lazy_static::lazy_static! {
    pub static ref ERROR_TYPE_CONFIGURATION: std::sync::Arc<ErrorTypeConfiguration> =
        ErrorTypeConfiguration {
            error_type_name: "Error".into(),
        }
        .into();
}

pub struct ErrorTypeConfiguration {
    pub error_type_name: String,
}

impl ErrorTypeConfiguration {
    pub fn qualify(&self, names: &HashMap<String, String>) -> Self {
        Self {
            error_type_name: self.qualify_name(&self.error_type_name, &names),
        }
    }

    fn qualify_name(&self, name: &str, names: &HashMap<String, String>) -> String {
        names.get(name).cloned().unwrap_or_else(|| name.into())
    }
}
