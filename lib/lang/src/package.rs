use serde::{Deserialize, Serialize};

// There is no relationship between Package and ExternalPackage as the former represents actual
// package information based on repository origins, commit hashes, etc. while the latter does
// package information which is expected by a build system and used for downloading the packages
// and computing dependency graphs.
//
// In the current design, we include versions in package data because we need to calculate external
// package paths used in import statements from them.
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
