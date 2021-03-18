use super::target::Target;
use crate::common::ExternalPackage;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct BuildConfiguration {
    target: Target,
    dependencies: HashSet<ExternalPackage>,
}

impl BuildConfiguration {
    pub fn new(target: Target, dependencies: HashSet<ExternalPackage>) -> Self {
        Self {
            target,
            dependencies,
        }
    }

    pub fn target(&self) -> &Target {
        &self.target
    }

    pub fn dependencies(&self) -> &HashSet<ExternalPackage> {
        &self.dependencies
    }
}
