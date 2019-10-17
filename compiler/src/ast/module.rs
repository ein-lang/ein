use super::definition::Definition;
use super::export::Export;
use super::expression::Expression;
use super::import::Import;
use crate::types::Type;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Module {
    definitions: Vec<Definition>,
    export: Export,
    imports: Vec<Import>,
}

impl Module {
    pub fn new(export: Export, imports: Vec<Import>, definitions: Vec<Definition>) -> Self {
        Self {
            definitions,
            export,
            imports,
        }
    }

    #[cfg(test)]
    pub fn without_exported_names(definitions: Vec<Definition>) -> Self {
        Self {
            definitions,
            export: Export::new(Default::default()),
            imports: vec![],
        }
    }

    pub fn definitions(&self) -> &[Definition] {
        &self.definitions
    }

    pub fn export(&self) -> &Export {
        &self.export
    }

    pub fn substitute_type_variables(&self, substitutions: &HashMap<usize, Type>) -> Self {
        Self::new(
            self.export.clone(),
            self.imports.clone(),
            self.definitions
                .iter()
                .map(|definition| definition.substitute_type_variables(substitutions))
                .collect::<Vec<_>>(),
        )
    }

    pub fn convert_definitions(&self, convert: &mut impl FnMut(&Definition) -> Definition) -> Self {
        Self::new(
            self.export.clone(),
            self.imports.clone(),
            self.definitions
                .iter()
                .map(|definition| definition.convert_definitions(convert))
                .collect(),
        )
    }

    pub fn convert_expressions(&self, convert: &mut impl FnMut(&Expression) -> Expression) -> Self {
        Self::new(
            self.export.clone(),
            self.imports.clone(),
            self.definitions
                .iter()
                .map(|definition| definition.convert_expressions(convert))
                .collect(),
        )
    }
}
