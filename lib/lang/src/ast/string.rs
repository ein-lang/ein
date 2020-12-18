use crate::debug::*;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct EinString {
    value: String,
    source_information: Arc<SourceInformation>,
}

impl EinString {
    pub fn new(
        value: impl Into<String>,
        source_information: impl Into<Arc<SourceInformation>>,
    ) -> Self {
        Self {
            value: value.into(),
            source_information: source_information.into(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn source_information(&self) -> &Arc<SourceInformation> {
        &self.source_information
    }
}
