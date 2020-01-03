use super::declaration::Declaration;
use super::definition::Definition;
use crate::types::canonicalize;

#[derive(Clone, Debug, PartialEq)]
pub struct Module {
    declarations: Vec<Declaration>,
    definitions: Vec<Definition>,
}

impl Module {
    pub fn new(declarations: Vec<Declaration>, definitions: Vec<Definition>) -> Self {
        Self {
            declarations,
            definitions,
        }
    }

    pub fn declarations(&self) -> &[Declaration] {
        &self.declarations
    }

    pub fn definitions(&self) -> &[Definition] {
        &self.definitions
    }

    pub fn canonicalize_types(&self) -> Self {
        Self {
            declarations: self
                .declarations
                .iter()
                .map(|declaration| declaration.convert_types(&canonicalize))
                .collect(),
            definitions: self
                .definitions
                .iter()
                .map(|definition| definition.convert_types(&canonicalize))
                .collect(),
        }
    }
}
