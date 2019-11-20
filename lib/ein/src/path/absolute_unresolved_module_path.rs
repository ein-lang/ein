use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AbsoluteUnresolvedModulePath {
    components: Vec<String>,
}

impl AbsoluteUnresolvedModulePath {
    pub fn new(components: Vec<String>) -> Self {
        Self { components }
    }

    pub fn components(&self) -> &[String] {
        &self.components
    }
}
