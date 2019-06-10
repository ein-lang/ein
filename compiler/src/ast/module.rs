use super::definition::Definition;

#[derive(Clone, Debug, PartialEq)]
pub struct Module {
    definitions: Vec<Definition>,
}

impl Module {
    pub fn new(definitions: Vec<Definition>) -> Self {
        Self { definitions }
    }

    pub fn definitions(&self) -> &[Definition] {
        &self.definitions
    }
}
