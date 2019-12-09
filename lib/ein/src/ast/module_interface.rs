use crate::path::ModulePath;
use crate::types::Type;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Deserialize, Serialize)]
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
