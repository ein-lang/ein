use super::external_package_configuration::ExternalPackageConfiguration;
use super::target::Target;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BuildConfiguration {
    pub target: Target,
    pub dependencies: HashMap<String, ExternalPackageConfiguration>,
}
