use super::external_package::ExternalPackage;

#[derive(Clone, Debug, PartialEq)]
pub struct ApplicationTarget {
    name: String,
    system_package: ExternalPackage,
}

impl ApplicationTarget {
    pub fn new(name: impl Into<String>, system_package: ExternalPackage) -> Self {
        Self {
            name: name.into(),
            system_package,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn system_package(&self) -> &ExternalPackage {
        &self.system_package
    }
}
