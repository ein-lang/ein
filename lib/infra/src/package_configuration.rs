use super::dependency_package::DependencyPackage;
use super::error::InfrastructureError;
use super::package_target::PackageTarget;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PackageConfiguration {
    name: String,
    version: semver::Version,
    target: PackageTarget,
    dependencies: Vec<DependencyPackage>,
}

impl PackageConfiguration {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &semver::Version {
        &self.version
    }

    pub fn target(&self) -> &PackageTarget {
        &self.target
    }

    #[allow(dead_code)]
    pub fn dependencies(&self) -> &[DependencyPackage] {
        &self.dependencies
    }

    pub fn verify(&self) -> Result<(), InfrastructureError> {
        self.target.verify()
    }
}
