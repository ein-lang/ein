use crate::debug::SourceInformation;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

static GLOBAL_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Variable {
    id: usize,
    source_information: Rc<SourceInformation>,
}

impl Variable {
    pub fn new(source_information: impl Into<Rc<SourceInformation>>) -> Self {
        Self {
            id: GLOBAL_ID.fetch_add(1, Ordering::SeqCst),
            source_information: source_information.into(),
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn source_information(&self) -> &Rc<SourceInformation> {
        &self.source_information
    }
}

impl PartialEq for Variable {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Hash for Variable {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.id.hash(hasher)
    }
}
