use crate::path::ModulePath;
use crate::types::Type;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ModuleInterface {
    path: ModulePath,
    types: HashMap<String, Type>,
}

impl ModuleInterface {
    pub fn new(path: ModulePath, types: HashMap<String, Type>) -> Self {
        Self { path, types }
    }

    pub fn path(&self) -> &ModulePath {
        &self.path
    }

    pub fn types(&self) -> &HashMap<String, Type> {
        &self.types
    }
}
