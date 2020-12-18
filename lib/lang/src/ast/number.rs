use crate::debug::SourceInformation;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct Number {
    value: f64,
    source_information: Arc<SourceInformation>,
}

impl Number {
    pub fn new(value: f64, source_information: impl Into<Arc<SourceInformation>>) -> Self {
        Self {
            value,
            source_information: source_information.into(),
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn source_information(&self) -> &Arc<SourceInformation> {
        &self.source_information
    }
}
