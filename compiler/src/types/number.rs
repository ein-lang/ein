use crate::debug::SourceInformation;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Number {
    source_information: Rc<SourceInformation>,
}

impl Number {
    pub fn new(source_information: impl Into<Rc<SourceInformation>>) -> Self {
        Self {
            source_information: source_information.into(),
        }
    }

    pub fn source_information(&self) -> &Rc<SourceInformation> {
        &self.source_information
    }
}
