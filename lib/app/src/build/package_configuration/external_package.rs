use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ExternalPackage {
    version: String,
}

impl ExternalPackage {
    #[cfg(test)]
    pub fn new(version: impl Into<String>) -> Self {
        Self {
            version: version.into(),
        }
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}
