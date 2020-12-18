use crate::debug::SourceInformation;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Any {
    source_information: Arc<SourceInformation>,
}

impl Any {
    pub fn new(source_information: impl Into<Arc<SourceInformation>>) -> Self {
        Self {
            source_information: source_information.into(),
        }
    }

    pub fn source_information(&self) -> &Arc<SourceInformation> {
        &self.source_information
    }
}
