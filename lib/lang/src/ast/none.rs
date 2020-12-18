use crate::debug::SourceInformation;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct None {
    source_information: Arc<SourceInformation>,
}

impl None {
    pub fn new(source_information: impl Into<Arc<SourceInformation>>) -> Self {
        Self {
            source_information: source_information.into(),
        }
    }

    pub fn source_information(&self) -> &Arc<SourceInformation> {
        &self.source_information
    }
}
