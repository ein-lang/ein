use crate::path::ModulePath;
use crate::types::Type;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ModuleInterface {
    path: ModulePath,
    variables: HashMap<String, Type>,
}

impl ModuleInterface {
    pub fn new(path: ModulePath, variables: HashMap<String, Type>) -> Self {
        Self { path, variables }
    }

    pub fn path(&self) -> &ModulePath {
        &self.path
    }

    pub fn variables(&self) -> &HashMap<String, Type> {
        &self.variables
    }
}
