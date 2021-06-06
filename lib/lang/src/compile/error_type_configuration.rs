#[cfg(test)]
use once_cell::sync::Lazy;
use std::collections::HashMap;
#[cfg(test)]
use std::sync::Arc;

#[cfg(test)]
pub static ERROR_TYPE_CONFIGURATION: Lazy<Arc<ErrorTypeConfiguration>> = Lazy::new(|| {
    ErrorTypeConfiguration {
        error_type_name: "Error".into(),
    }
    .into()
});

pub struct ErrorTypeConfiguration {
    pub error_type_name: String,
}

impl ErrorTypeConfiguration {
    pub fn qualify(&self, names: &HashMap<String, String>) -> Self {
        Self {
            error_type_name: self.qualify_name(&self.error_type_name, names),
        }
    }

    fn qualify_name(&self, name: &str, names: &HashMap<String, String>) -> String {
        names.get(name).cloned().unwrap_or_else(|| name.into())
    }
}
