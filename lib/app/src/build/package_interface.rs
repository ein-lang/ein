use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PackageInterface {
    modules: Vec<lang::ModuleInterface>,
}

impl PackageInterface {
    pub fn new(modules: &[lang::ModuleInterface]) -> Self {
        Self {
            modules: modules.to_vec(),
        }
    }

    pub fn modules(&self) -> &[lang::ModuleInterface] {
        &self.modules
    }
}
