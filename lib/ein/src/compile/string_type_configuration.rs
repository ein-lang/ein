#[cfg(test)]
use lazy_static::lazy_static;
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
