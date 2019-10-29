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

    pub fn types<'a>(&'a self) -> impl 'a + Iterator<Item = (String, &'a Type)> {
        self.types.iter().map(move |(name, type_)| {
            (
                [
                    self.path.components().iter().last().unwrap().as_str(),
                    &name,
                ]
                .join("."),
                type_,
            )
        })
    }

    pub fn fully_qualified_types<'a>(&'a self) -> impl 'a + Iterator<Item = (String, &'a Type)> {
        self.types.iter().map(move |(name, type_)| {
            (
                self.path
                    .components()
                    .iter()
                    .map(|component| component.as_str())
                    .chain(vec![name.as_str()])
                    .collect::<Vec<_>>()
                    .join("."),
                type_,
            )
        })
    }
}
