#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ExternalPackage {
    name: String,
    // This version is the "expected" one different from lang::Package.
    version: String,
}

impl ExternalPackage {
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
