use crate::{path::ModulePath, types::Type};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct ModuleInterface {
    path: ModulePath,
    exported_names: BTreeSet<String>,  // Unqualified
    types: BTreeMap<String, Type>,     // Fully-qualified
    variables: BTreeMap<String, Type>, // Fully-qualified
}

impl ModuleInterface {
    pub fn new(
        path: ModulePath,
        exported_names: BTreeSet<String>,
        types: BTreeMap<String, Type>,
        variables: BTreeMap<String, Type>,
    ) -> Self {
        Self {
            path,
            exported_names,
            types,
            variables,
        }
    }

    pub fn path(&self) -> &ModulePath {
        &self.path
    }

    pub fn exported_names(&self) -> &BTreeSet<String> {
        &self.exported_names
    }

    pub fn types(&self) -> &BTreeMap<String, Type> {
        &self.types
    }

    pub fn variables(&self) -> &BTreeMap<String, Type> {
        &self.variables
    }
}
