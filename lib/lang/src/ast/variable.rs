use crate::debug::SourceInformation;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct Variable {
    name: String,
    source_information: Arc<SourceInformation>,
}

impl Variable {
    pub fn new(
        name: impl Into<String>,
        source_information: impl Into<Arc<SourceInformation>>,
    ) -> Self {
        Self {
            name: name.into(),
            source_information: source_information.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn source_information(&self) -> &Arc<SourceInformation> {
        &self.source_information
    }
}
