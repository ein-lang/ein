use crate::types;
use std::collections::HashMap;

#[cfg(test)]
lazy_static::lazy_static! {
    pub static ref BUILTIN_CONFIGURATION: std::sync::Arc<BuiltinConfiguration> = BuiltinConfiguration {
        functions: Default::default(),
    }
    .into();
}

pub struct BuiltinConfiguration {
    pub functions: HashMap<String, types::Function>,
}
