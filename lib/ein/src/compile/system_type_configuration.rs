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
