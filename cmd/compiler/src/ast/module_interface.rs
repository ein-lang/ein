use crate::types::Type;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ModuleInterface {
    types: HashMap<String, Type>,
}

impl ModuleInterface {
    pub fn new(types: HashMap<String, Type>) -> Self {
        Self { types }
    }

    pub fn types(&self) -> &HashMap<String, Type> {
        &self.types
    }
}
