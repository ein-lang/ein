use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
    pub static ref BUILTIN_CONFIGURATION: Arc<lang::BuiltinConfiguration> =
        lang::BuiltinConfiguration {
            functions: vec![].into_iter().collect(),
        }
        .into();
}
