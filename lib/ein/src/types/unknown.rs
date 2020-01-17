use crate::debug::SourceInformation;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Unknown {
    source_information: Rc<SourceInformation>,
}

impl Unknown {
    pub fn new(source_information: impl Into<Rc<SourceInformation>>) -> Self {
        Self {
            source_information: source_information.into(),
        }
    }

    pub fn source_information(&self) -> &Rc<SourceInformation> {
        &self.source_information
    }
}
