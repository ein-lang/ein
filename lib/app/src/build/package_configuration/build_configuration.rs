use super::external_package::ExternalPackage;
use super::target::Target;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BuildConfiguration {
    target: Target,
    dependencies: HashMap<String, ExternalPackage>,
}

impl BuildConfiguration {
    pub fn new(target: Target, dependencies: HashMap<String, ExternalPackage>) -> Self {
        Self {
            target,
            dependencies,
        }
    }

    pub fn target(&self) -> &Target {
        &self.target
    }

    pub fn dependencies(&self) -> &HashMap<String, ExternalPackage> {
        &self.dependencies
    }
}
