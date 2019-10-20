use super::definition::Definition;
use super::export::Export;
use super::import::Import;

#[derive(Clone, Debug, PartialEq)]
pub struct UnresolvedModule {
    definitions: Vec<Definition>,
    export: Export,
    imports: Vec<Import>,
}

impl UnresolvedModule {
    pub fn new(export: Export, imports: Vec<Import>, definitions: Vec<Definition>) -> Self {
        Self {
            definitions,
            export,
            imports,
        }
    }

    #[cfg(test)]
    pub fn from_definitions(definitions: Vec<Definition>) -> Self {
        Self::new(Export::new(Default::default()), vec![], definitions)
    }

    pub fn definitions(&self) -> &[Definition] {
        &self.definitions
    }

    pub fn export(&self) -> &Export {
        &self.export
    }

    pub fn imports(&self) -> &[Import] {
        &self.imports
    }
}
