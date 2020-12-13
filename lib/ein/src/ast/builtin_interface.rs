use crate::types::{self, Type};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub struct BuiltinInterface {
    types: BTreeMap<String, Type>,
    functions: BTreeMap<String, types::Function>,
}

impl BuiltinInterface {
    pub fn new(
        types: BTreeMap<String, Type>,
        functions: BTreeMap<String, types::Function>,
    ) -> Self {
        Self { types, functions }
    }

    #[cfg(test)]
    pub fn dummy() -> Self {
        Self {
            types: Default::default(),
            functions: Default::default(),
        }
    }

    pub fn types(&self) -> &BTreeMap<String, Type> {
        &self.types
    }

    pub fn functions(&self) -> &BTreeMap<String, types::Function> {
        &self.functions
    }
}
