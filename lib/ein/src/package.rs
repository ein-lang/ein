use serde::{Deserialize, Serialize};

// In the current design, there is no relationship between Package and
// ExternalPackage as the former represents actual package information based on
// repository origins, commit hashes, etc. while the latter does expected
// package information which is used only for downloading the packages and
// computing dependency graphs.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Package {
    name: String,
    version: String,
}

impl Package {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}
