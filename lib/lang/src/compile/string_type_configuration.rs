#[cfg(test)]
use lazy_static::lazy_static;
use std::collections::HashMap;
#[cfg(test)]
use std::sync::Arc;

#[cfg(test)]
lazy_static! {
    pub static ref STRING_TYPE_CONFIGURATION: Arc<StringTypeConfiguration> =
        StringTypeConfiguration {
            equal_function_name: "equalStrings".into(),
        }
        .into();
}

pub struct StringTypeConfiguration {
    pub equal_function_name: String,
}

impl StringTypeConfiguration {
    pub fn qualify(&self, names: &HashMap<String, String>) -> Self {
        Self {
            equal_function_name: self.qualify_name(&self.equal_function_name, &names),
        }
    }

    fn qualify_name(&self, name: &str, names: &HashMap<String, String>) -> String {
        names.get(name).cloned().unwrap_or_else(|| name.into())
    }
}
