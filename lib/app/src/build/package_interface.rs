use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PackageInterface {
    modules: Vec<ein::ModuleInterface>,
}

impl PackageInterface {
    pub fn new(modules: &[ein::ModuleInterface]) -> Self {
        Self {
            modules: modules.to_vec(),
        }
    }

    pub fn modules(&self) -> &[ein::ModuleInterface] {
        &self.modules
    }
}
