#[derive(Clone, Debug)]
pub struct CommandTarget {
    name: String,
}

impl CommandTarget {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
