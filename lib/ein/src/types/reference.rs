use crate::debug::SourceInformation;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Reference {
    name: String,
    source_information: Rc<SourceInformation>,
}

impl Reference {
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
