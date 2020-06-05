use super::external_package_configuration::ExternalPackageConfiguration;
use super::target::Target;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BuildConfiguration {
    target: Target,
    dependencies: HashMap<String, ExternalPackageConfiguration>,
}

impl BuildConfiguration {
    pub fn new(
        target: Target,
        dependencies: HashMap<String, ExternalPackageConfiguration>,
    ) -> Self {
        Self {
            target,
            dependencies,
        }
    }

    pub fn target(&self) -> &Target {
        &self.target
    }

    pub fn dependencies(&self) -> &HashMap<String, ExternalPackageConfiguration> {
        &self.dependencies
    }
}
