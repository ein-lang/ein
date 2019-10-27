use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RawPackageConfiguration {
    name: String,
    version: String,
}

impl RawPackageConfiguration {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}
