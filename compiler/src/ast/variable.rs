use crate::debug::SourceInformation;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Variable {
    name: String,
    source_information: Rc<SourceInformation>,
}

impl Variable {
    pub fn new(
        name: impl Into<String>,
        source_information: impl Into<Rc<SourceInformation>>,
    ) -> Self {
        Self {
            name: name.into(),
            source_information: source_information.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn source_information(&self) -> &Rc<SourceInformation> {
        &self.source_information
    }
}
