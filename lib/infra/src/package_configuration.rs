use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PackageConfiguration {
    name: String,
    version: String,
}

impl PackageConfiguration {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}
