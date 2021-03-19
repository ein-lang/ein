use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct JsonExternalPackageConfiguration {
    version: String,
}

impl JsonExternalPackageConfiguration {
    pub fn new(version: impl Into<String>) -> Self {
        Self {
            version: version.into(),
        }
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}
