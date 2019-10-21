use super::definition::Definition;
use super::export::Export;
use super::expression::Expression;
use super::module_interface::ModuleInterface;
use crate::path::ModulePath;
use crate::types::Type;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Module {
    path: ModulePath,
    definitions: Vec<Definition>,
    export: Export,
    imported_modules: Vec<ModuleInterface>,
}

impl Module {
    pub fn new(
        path: ModulePath,
        export: Export,
        imported_modules: Vec<ModuleInterface>,
        definitions: Vec<Definition>,
    ) -> Self {
        Self {
            path: path.into(),
            definitions,
            export,
            imported_modules,
        }
    }

    #[cfg(test)]
    pub fn from_definitions(definitions: Vec<Definition>) -> Self {
        Self::new(
            ModulePath::Relative(vec![]),
            Export::new(Default::default()),
            vec![],
            definitions,
        )
    }

    pub fn path(&self) -> &ModulePath {
        &self.path
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

    pub fn substitute_type_variables(&self, substitutions: &HashMap<usize, Type>) -> Self {
        Self::new(
            self.path.clone(),
            self.export.clone(),
            self.imported_modules.clone(),
            self.definitions
                .iter()
                .map(|definition| definition.substitute_type_variables(substitutions))
                .collect::<Vec<_>>(),
        )
    }

    pub fn convert_definitions(&self, convert: &mut impl FnMut(&Definition) -> Definition) -> Self {
        Self::new(
            self.path.clone(),
            self.export.clone(),
            self.imported_modules.clone(),
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
            self.definitions
                .iter()
                .map(|definition| definition.convert_expressions(convert))
                .collect(),
        )
    }
}
