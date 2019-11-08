use crate::debug::SourceInformation;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Number {
    value: f64,
    source_information: Rc<SourceInformation>,
}

impl Number {
    pub fn new(value: f64, source_information: impl Into<Rc<SourceInformation>>) -> Self {
        Self {
            value,
            source_information: source_information.into(),
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn source_information(&self) -> &Rc<SourceInformation> {
        &self.source_information
    }
}
