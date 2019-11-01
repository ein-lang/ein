use super::module_path::ModulePath;
use crate::package::Package;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct UnresolvedModulePath {
    package_name: String,
    components: Vec<String>,
}

impl UnresolvedModulePath {
    pub fn new(package_name: impl Into<String>, components: Vec<String>) -> Self {
        Self {
            package_name: package_name.into(),
            components,
        }
    }

    pub fn package_name(&self) -> &str {
        &self.package_name
    }

    pub fn components(&self) -> &[String] {
        &self.components
    }

    pub fn resolve_version(self, version: impl Into<semver::Version>) -> ModulePath {
        ModulePath::new(Package::new(self.package_name, version), self.components)
    }
}
