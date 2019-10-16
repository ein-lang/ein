use super::definition::Definition;
use super::expression::Expression;
use super::import::Import;
use crate::types::Type;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq)]
pub struct Module {
    definitions: Vec<Definition>,
    exported_names: HashSet<String>,
    imports: Vec<Import>,
}

impl Module {
    pub fn new(
        exported_names: HashSet<String>,
        imports: Vec<Import>,
        definitions: Vec<Definition>,
    ) -> Self {
        Self {
            definitions,
            exported_names,
            imports,
        }
    }

    #[cfg(test)]
    pub fn without_exported_names(definitions: Vec<Definition>) -> Self {
        Self {
            definitions,
            exported_names: Default::default(),
            imports: vec![],
        }
    }

    pub fn definitions(&self) -> &[Definition] {
        &self.definitions
    }

    pub fn exported_names(&self) -> &HashSet<String> {
        &self.exported_names
    }

    pub fn substitute_type_variables(&self, substitutions: &HashMap<usize, Type>) -> Self {
        Self::new(
            self.exported_names.clone(),
            self.imports.clone(),
            self.definitions
                .iter()
                .map(|definition| definition.substitute_type_variables(substitutions))
                .collect::<Vec<_>>(),
        )
    }

    pub fn convert_definitions(&self, convert: &mut impl FnMut(&Definition) -> Definition) -> Self {
        Self::new(
            self.exported_names.clone(),
            self.imports.clone(),
            self.definitions
                .iter()
                .map(|definition| definition.convert_definitions(convert))
                .collect(),
        )
    }

    pub fn convert_expressions(&self, convert: &mut impl FnMut(&Expression) -> Expression) -> Self {
        Self::new(
            self.exported_names.clone(),
            self.imports.clone(),
            self.definitions
                .iter()
                .map(|definition| definition.convert_expressions(convert))
                .collect(),
        )
    }
}
