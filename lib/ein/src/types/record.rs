use super::Type;
use crate::debug::SourceInformation;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Record {
    elements: BTreeMap<String, Type>,
    source_information: Rc<SourceInformation>,
}

impl Record {
    pub fn new(
        elements: BTreeMap<String, Type>,
        source_information: impl Into<Rc<SourceInformation>>,
    ) -> Self {
        Self {
            elements,
            source_information: source_information.into(),
        }
    }

    pub fn elements(&self) -> &BTreeMap<String, Type> {
        &self.elements
    }

    pub fn source_information(&self) -> &Rc<SourceInformation> {
        &self.source_information
    }

    pub fn substitute_variables(&self, substitutions: &HashMap<usize, Type>) -> Self {
        Self::new(
            self.elements
                .iter()
                .map(|(name, type_)| (name.into(), type_.substitute_variables(substitutions)))
                .collect(),
            self.source_information.clone(),
        )
    }

    pub fn resolve_reference_types(&self, environment: &HashMap<String, Type>) -> Self {
        Self::new(
            self.elements
                .iter()
                .map(|(name, type_)| (name.into(), type_.resolve_reference_types(environment)))
                .collect(),
            self.source_information.clone(),
        )
    }
}
