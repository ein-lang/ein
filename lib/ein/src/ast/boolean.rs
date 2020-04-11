use crate::debug::SourceInformation;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Boolean {
    value: bool,
    source_information: Rc<SourceInformation>,
}

impl Boolean {
    pub fn new(value: bool, source_information: impl Into<Rc<SourceInformation>>) -> Self {
        Self {
            value,
            source_information: source_information.into(),
        }
    }

    pub fn value(&self) -> bool {
        self.value
    }

    pub fn source_information(&self) -> &Rc<SourceInformation> {
        &self.source_information
    }
}
