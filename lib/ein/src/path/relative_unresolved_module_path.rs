use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RelativeUnresolvedModulePath {
    components: Vec<String>,
}

impl RelativeUnresolvedModulePath {
    pub fn new(components: Vec<String>) -> Self {
        Self { components }
    }

    pub fn components(&self) -> &[String] {
        &self.components
    }
}
