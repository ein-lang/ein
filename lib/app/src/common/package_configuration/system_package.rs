#[derive(Clone, Debug, Hash, PartialEq)]
pub struct SystemPackage {
    name: String,
    version: String,
}

impl SystemPackage {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}
