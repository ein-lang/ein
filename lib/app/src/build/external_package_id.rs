#[derive(Clone, Eq, Hash, PartialEq)]
pub struct ExternalPackageId {
    name: String,
    version: String,
}

impl ExternalPackageId {
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
