use super::definition::Definition;
use super::export::Export;
use super::import::Import;
use super::import_foreign::ImportForeign;
use super::module::Module;
use super::type_definition::TypeDefinition;
use super::unresolved_import::UnresolvedImport;
use crate::path::ModulePath;

#[derive(Clone, Debug, PartialEq)]
pub struct UnresolvedModule {
    type_definitions: Vec<TypeDefinition>,
    definitions: Vec<Definition>,
    export: Export,
    imports: Vec<UnresolvedImport>,
    import_foreigns: Vec<ImportForeign>,
}

impl UnresolvedModule {
    pub fn new(
        export: Export,
        imports: Vec<UnresolvedImport>,
        import_foreigns: Vec<ImportForeign>,
        type_definitions: Vec<TypeDefinition>,
        definitions: Vec<Definition>,
    ) -> Self {
        Self {
            type_definitions,
            definitions,
            export,
            imports,
            import_foreigns,
        }
    }

    pub fn resolve(self, path: ModulePath, imports: Vec<Import>) -> Module {
        Module::new(
            path,
            self.export,
            imports,
            self.import_foreigns,
            self.type_definitions,
            self.definitions,
        )
    }

    #[cfg(test)]
    pub fn from_definitions(definitions: Vec<Definition>) -> Self {
        Self::new(
            Export::new(Default::default()),
            vec![],
            vec![],
            vec![],
            definitions,
        )
    }

    pub fn definitions(&self) -> &[Definition] {
        &self.definitions
    }

    pub fn export(&self) -> &Export {
        &self.export
    }

    pub fn imports(&self) -> &[UnresolvedImport] {
        &self.imports
    }

    pub fn import_foreigns(&self) -> &[ImportForeign] {
        &self.import_foreigns
    }
}
