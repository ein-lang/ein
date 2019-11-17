use super::dependency_package::DependencyPackage;
use super::target::Target;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct PackageConfiguration {
    name: String,
    version: semver::Version,
    target: Target,
    dependencies: HashMap<String, DependencyPackage>,
}

impl PackageConfiguration {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &semver::Version {
        &self.version
    }

    pub fn target(&self) -> &Target {
        &self.target
    }

    #[allow(dead_code)]
    pub fn dependencies(&self) -> &HashMap<String, DependencyPackage> {
        &self.dependencies
    }
}
