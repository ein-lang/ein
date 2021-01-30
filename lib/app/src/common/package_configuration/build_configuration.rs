use super::external_package_configuration::ExternalPackageConfiguration;
use super::target::Target;
use crate::common::ExternalPackage;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// TODO Put this in the adaptor layer.
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

    pub fn dependencies(&self) -> impl IntoIterator<Item = ExternalPackage> + '_ {
        self.dependencies
            .iter()
            .map(|(name, configuration)| ExternalPackage::new(name, configuration.version()))
    }
}
