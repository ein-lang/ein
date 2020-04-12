use super::Type;
use crate::debug::SourceInformation;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Union {
    types: Vec<Type>,
    source_information: Rc<SourceInformation>,
}

impl Union {
    pub fn new(types: Vec<Type>, source_information: impl Into<Rc<SourceInformation>>) -> Self {
        Self {
            types,
            source_information: source_information.into(),
        }
    }

    pub fn types(&self) -> &[Type] {
        &self.types
    }

    pub fn source_information(&self) -> &Rc<SourceInformation> {
        &self.source_information
    }

    pub fn substitute_variables(&self, substitutions: &HashMap<usize, Type>) -> Self {
        Self::new(
            self.types
                .iter()
                .map(|type_| type_.substitute_variables(substitutions))
                .collect(),
            self.source_information.clone(),
        )
    }
}
