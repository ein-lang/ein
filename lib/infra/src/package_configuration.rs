use super::external_package::ExternalPackage;
use super::target::Target;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct PackageConfiguration {
    target: Target,
    dependencies: HashMap<String, ExternalPackage>,
}

impl PackageConfiguration {
    pub fn target(&self) -> &Target {
        &self.target
    }

    pub fn dependencies(&self) -> &HashMap<String, ExternalPackage> {
        &self.dependencies
    }
}
