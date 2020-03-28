use super::definition::Definition;
use super::export::Export;
use super::expression::Expression;
use super::module_interface::ModuleInterface;
use super::type_definition::TypeDefinition;
use crate::path::ModulePath;
use crate::types::Type;

#[derive(Clone, Debug, PartialEq)]
pub struct Module {
    path: ModulePath,
    type_definitions: Vec<TypeDefinition>,
    definitions: Vec<Definition>,
    export: Export,
    imported_modules: Vec<ModuleInterface>,
}

impl Module {
    pub fn new(
        path: ModulePath,
        export: Export,
        imported_modules: Vec<ModuleInterface>,
        type_definitions: Vec<TypeDefinition>,
        definitions: Vec<Definition>,
    ) -> Self {
        Self {
            path,
            type_definitions,
            definitions,
            export,
            imported_modules,
        }
    }

    #[cfg(test)]
    pub fn dummy() -> Self {
        Self::new(
            ModulePath::new(crate::package::Package::new("", ""), vec![]),
            Export::new(Default::default()),
            vec![],
            vec![],
            vec![],
        )
    }

    #[cfg(test)]
    pub fn from_definitions(definitions: Vec<Definition>) -> Self {
        Self::new(
            ModulePath::new(crate::package::Package::new("", ""), vec![]),
            Export::new(Default::default()),
            vec![],
            vec![],
            definitions,
        )
    }

    #[cfg(test)]
    pub fn from_definitions_and_type_definitions(
        type_definitions: Vec<TypeDefinition>,
        definitions: Vec<Definition>,
    ) -> Self {
        Self::new(
            ModulePath::new(crate::package::Package::new("", ""), vec![]),
            Export::new(Default::default()),
            vec![],
            type_definitions,
            definitions,
        )
    }

    pub fn path(&self) -> &ModulePath {
        &self.path
    }

    pub fn type_definitions(&self) -> &[TypeDefinition] {
        &self.type_definitions
    }

    pub fn definitions(&self) -> &[Definition] {
        &self.definitions
    }

    pub fn export(&self) -> &Export {
        &self.export
    }

    pub fn imported_modules(&self) -> &[ModuleInterface] {
        &self.imported_modules
    }

    pub fn convert_definitions(&self, convert: &mut impl FnMut(&Definition) -> Definition) -> Self {
        Self::new(
            self.path.clone(),
            self.export.clone(),
            self.imported_modules.clone(),
            self.type_definitions.clone(),
            self.definitions
                .iter()
                .map(|definition| definition.convert_definitions(convert))
                .collect(),
        )
    }

    pub fn convert_expressions(&self, convert: &mut impl FnMut(&Expression) -> Expression) -> Self {
        Self::new(
            self.path.clone(),
            self.export.clone(),
            self.imported_modules.clone(),
            self.type_definitions.clone(),
            self.definitions
                .iter()
                .map(|definition| definition.convert_expressions(convert))
                .collect(),
        )
    }

    pub fn convert_types(&self, convert: &mut impl FnMut(&Type) -> Type) -> Self {
        Self::new(
            self.path.clone(),
            self.export.clone(),
            self.imported_modules.clone(),
            self.type_definitions.clone(),
            self.definitions
                .iter()
                .map(|definition| definition.convert_types(convert))
                .collect(),
        )
    }
}
