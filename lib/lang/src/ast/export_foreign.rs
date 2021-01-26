use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq)]
pub struct ExportForeign {
    names: HashSet<String>,
}

impl ExportForeign {
    pub fn new(names: HashSet<String>) -> Self {
        Self { names }
    }

    pub fn names(&self) -> &HashSet<String> {
        &self.names
    }
}
