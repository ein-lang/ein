#[derive(Clone, Debug, PartialEq)]
pub struct RelativeModulePath {
    components: Vec<String>,
}

impl RelativeModulePath {
    pub fn new(components: Vec<String>) -> Self {
        Self { components }
    }

    pub fn components(&self) -> &[String] {
        &self.components
    }
}
