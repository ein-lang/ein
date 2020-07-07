use crate::debug::SourceInformation;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct Boolean {
    value: bool,
    source_information: Arc<SourceInformation>,
}

impl Boolean {
    pub fn new(value: bool, source_information: impl Into<Arc<SourceInformation>>) -> Self {
        Self {
            value,
            source_information: source_information.into(),
        }
    }

    pub fn value(&self) -> bool {
        self.value
    }

    pub fn source_information(&self) -> &Arc<SourceInformation> {
        &self.source_information
    }
}
