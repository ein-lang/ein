mod command_target;
mod error;
mod external_package;
mod target;
mod target_type;

pub use command_target::CommandTarget;
pub use external_package::ExternalPackage;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub use target::Target;

#[derive(Deserialize, Serialize)]
pub struct PackageConfiguration {
    target: Target,
    dependencies: HashMap<String, ExternalPackage>,
}

impl PackageConfiguration {
    #[cfg(test)]
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
