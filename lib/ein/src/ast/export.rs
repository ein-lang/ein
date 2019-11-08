use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq)]
pub struct Export {
    names: HashSet<String>,
}

impl Export {
    pub fn new(names: HashSet<String>) -> Self {
        Self { names }
    }

    pub fn names(&self) -> &HashSet<String> {
        &self.names
    }
}
