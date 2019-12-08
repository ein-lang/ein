use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PackageInterface {
    modules: Vec<ein::ast::ModuleInterface>,
}

impl PackageInterface {
    pub fn new(modules: &[ein::ast::ModuleInterface]) -> Self {
        Self {
            modules: modules.to_vec(),
        }
    }

    pub fn modules(&self) -> &[ein::ast::ModuleInterface] {
        &self.modules
    }
}
