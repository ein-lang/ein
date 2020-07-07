use crate::debug::SourceInformation;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// TODO: Consider renaming this "Alias".
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Reference {
    name: String,
    source_information: Arc<SourceInformation>,
}

impl Reference {
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
