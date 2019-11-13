use super::dependency_package::DependencyPackage;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PackageConfiguration {
    name: String,
    version: semver::Version,
    #[serde(rename(deserialize = "exposedModules"))]
    exposed_modules: Vec<String>,
    dependencies: Vec<DependencyPackage>,
}

impl PackageConfiguration {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &semver::Version {
        &self.version
    }

    #[allow(dead_code)]
    pub fn exposed_modules(&self) -> &[String] {
        &self.exposed_modules
    }

    #[allow(dead_code)]
    pub fn dependencies(&self) -> &[DependencyPackage] {
        &self.dependencies
    }
}
