use super::json_system_package_configuration::JsonSystemPackageConfiguration;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct JsonApplicationBuildConfiguration {
    name: String,
    system: JsonSystemPackageConfiguration,
}

impl JsonApplicationBuildConfiguration {
    pub fn new(name: impl Into<String>, system: JsonSystemPackageConfiguration) -> Self {
        Self {
            name: name.into(),
            system,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn system(&self) -> &JsonSystemPackageConfiguration {
        &self.system
    }
}
