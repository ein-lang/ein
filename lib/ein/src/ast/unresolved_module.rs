use super::definition::Definition;
use super::export::Export;
use super::import::Import;
use super::module::Module;
use super::module_interface::ModuleInterface;
use crate::path::ModulePath;

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

    pub fn resolve(self, path: ModulePath, module_interfaces: Vec<ModuleInterface>) -> Module {
        Module::new(path, self.export, module_interfaces, self.definitions)
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
