use super::system_package_configuration::SystemPackageConfiguration;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CommandTarget {
    name: String,
    #[serde(rename = "systemPackage")]
    system_package: SystemPackageConfiguration,
}

impl CommandTarget {
    pub fn new(name: impl Into<String>, system_package: SystemPackageConfiguration) -> Self {
        Self {
            name: name.into(),
            system_package,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn system_package(&self) -> &SystemPackageConfiguration {
        &self.system_package
    }
}
