use super::pattern::Pattern;
use crate::debug::SourceInformation;
use crate::types::Type;
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct RecordPattern {
    type_: Type, // Must be record type
    elements: BTreeMap<String, Pattern>,
    source_information: Rc<SourceInformation>,
}

impl RecordPattern {
    pub fn new(
        type_: impl Into<Type>,
        elements: BTreeMap<String, Pattern>,
        source_information: impl Into<Rc<SourceInformation>>,
    ) -> Self {
        Self {
            type_: type_.into(),
            elements,
            source_information: source_information.into(),
        }
    }

    pub fn type_(&self) -> &Type {
        &self.type_
    }

    pub fn elements(&self) -> &BTreeMap<String, Pattern> {
        &self.elements
    }

    pub fn source_information(&self) -> &Rc<SourceInformation> {
        &self.source_information
    }
}
