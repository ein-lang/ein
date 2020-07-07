use crate::debug::SourceInformation;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

static GLOBAL_VARIABLE_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Variable {
    id: usize,
    source_information: Arc<SourceInformation>,
}

impl Variable {
    pub fn new(source_information: impl Into<Arc<SourceInformation>>) -> Self {
        Self {
            id: GLOBAL_VARIABLE_ID.fetch_add(1, Ordering::SeqCst),
            source_information: source_information.into(),
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn source_information(&self) -> &Arc<SourceInformation> {
        &self.source_information
    }
}
