use crate::path::ModulePath;
use crate::types::Type;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ModuleInterface {
    path: ModulePath,
    types: HashMap<String, Type>,
    name_map: HashMap<String, String>,
}

impl ModuleInterface {
    pub fn new(path: ModulePath, types: HashMap<String, Type>) -> Self {
        Self {
            name_map: types
                .keys()
                .map(|key| (path.qualify_name(key), path.fully_qualify_name(key)))
                .collect(),
            path,
            types,
        }
    }

    pub fn path(&self) -> &ModulePath {
        &self.path
    }

    pub fn types<'a>(&'a self) -> impl 'a + Iterator<Item = (String, &'a Type)> {
        self.types
            .iter()
            .map(move |(name, type_)| (self.path.qualify_name(name), type_))
    }

    pub fn name_map(&self) -> &HashMap<String, String> {
        &self.name_map
    }
}
