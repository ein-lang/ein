use crate::path::ModulePath;
use crate::types::Type;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ModuleInterface {
    path: ModulePath,
    types: HashMap<String, Type>,
    variables: HashMap<String, Type>,
}

impl ModuleInterface {
    pub fn new(
        path: ModulePath,
        types: HashMap<String, Type>,
        variables: HashMap<String, Type>,
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

    pub fn types(&self) -> &HashMap<String, Type> {
        &self.types
    }

    pub fn variables(&self) -> &HashMap<String, Type> {
        &self.variables
    }
}

impl PartialEq for ModuleInterface {
    fn eq(&self, another: &Self) -> bool {
        self.path == another.path && self.variables == another.variables
    }
}

impl Hash for ModuleInterface {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.path.hash(hasher);

        self.variables
            .iter()
            .collect::<BTreeMap<_, _>>()
            .hash(hasher);
    }
}
