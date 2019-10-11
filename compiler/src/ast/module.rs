use super::definition::Definition;
use crate::types::Type;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Module {
    definitions: Vec<Definition>,
}

impl Module {
    pub fn new(definitions: Vec<Definition>) -> Self {
        Self { definitions }
    }

    pub fn definitions(&self) -> &[Definition] {
        &self.definitions
    }

    pub fn substitute_type_variables(&self, substitutions: &HashMap<usize, Type>) -> Self {
        Self::new(
            self.definitions
                .iter()
                .map(|definition| definition.substitute_type_variables(substitutions))
                .collect::<Vec<_>>(),
        )
    }

    pub fn convert_definitions(&self, convert: &mut impl FnMut(&Definition) -> Definition) -> Self {
        Self::new(
            self.definitions
                .iter()
                .map(|definition| definition.convert_definitions(convert))
                .collect(),
        )
    }
}
