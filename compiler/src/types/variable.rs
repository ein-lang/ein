use crate::types::Type;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

static GLOBAL_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Debug)]
pub struct Variable {
    id: usize,
}

impl Variable {
    pub fn new() -> Self {
        Self {
            id: GLOBAL_ID.fetch_add(1, Ordering::SeqCst),
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn substitute_variables(&self, substitutions: &HashMap<usize, Type>) -> Type {
        match substitutions.get(&self.id) {
            Some(type_) => type_.clone(),
            None => self.clone().into(),
        }
    }
}

impl PartialEq for Variable {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}
