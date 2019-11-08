use super::module_path::ModulePath;
use crate::package::Package;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct UnresolvedModulePath {
    components: Vec<String>,
}

impl UnresolvedModulePath {
    pub fn new(components: Vec<String>) -> Self {
        Self { components }
    }

    pub fn components(&self) -> &[String] {
        &self.components
    }

    pub fn resolve(self, package: Package) -> ModulePath {
        ModulePath::new(package, self.components)
    }
}
