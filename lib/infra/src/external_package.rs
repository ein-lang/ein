use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ExternalPackage {
    version: String,
}

impl ExternalPackage {
    pub fn version(&self) -> &str {
        &self.version
    }
}
