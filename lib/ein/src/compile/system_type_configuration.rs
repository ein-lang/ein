use std::collections::HashMap;

#[cfg(test)]
lazy_static::lazy_static! {
    pub static ref SYSTEM_TYPE_CONFIGURATION: std::sync::Arc<SystemTypeConfiguration> =
        SystemTypeConfiguration {
            system_type_name: "System".into(),
        }
        .into();
}

pub struct SystemTypeConfiguration {
    pub system_type_name: String,
}

impl SystemTypeConfiguration {
    pub fn qualify(&self, names: &HashMap<String, String>) -> Self {
        Self {
            system_type_name: self.qualify_name(&self.system_type_name, &names),
        }
    }

    fn qualify_name(&self, name: &str, names: &HashMap<String, String>) -> String {
        names.get(name).cloned().unwrap_or_else(|| name.into())
    }
}
