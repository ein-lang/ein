use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ExternalUnresolvedModulePath {
    components: Vec<String>,
}

impl ExternalUnresolvedModulePath {
    pub fn new(components: Vec<String>) -> Self {
        Self { components }
    }

    pub fn components(&self) -> impl Iterator<Item = &str> {
        self.components.iter().map(Deref::deref)
    }
}

impl std::fmt::Display for ExternalUnresolvedModulePath {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self.components.join("/"))
    }
}
