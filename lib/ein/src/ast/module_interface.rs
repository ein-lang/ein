use crate::path::ModulePath;
use crate::types::Type;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct ModuleInterface {
    path: ModulePath,
    types: BTreeMap<String, Type>,
    variables: BTreeMap<String, Type>,
}

impl ModuleInterface {
    pub fn new(
        path: ModulePath,
        types: BTreeMap<String, Type>,
        variables: BTreeMap<String, Type>,
    ) -> Self {
        Self {
            path,
            types,
            variables,
        }
    }

    pub fn path(&self) -> &ModulePath {
        &self.path
    }

    pub fn types(&self) -> &BTreeMap<String, Type> {
        &self.types
    }

    pub fn variables(&self) -> &BTreeMap<String, Type> {
        &self.variables
    }
}
