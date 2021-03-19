use super::system_package::SystemPackage;

#[derive(Clone, Debug, PartialEq)]
pub struct ApplicationTarget {
    name: String,
    system_package: SystemPackage,
}

impl ApplicationTarget {
    pub fn new(name: impl Into<String>, system_package: SystemPackage) -> Self {
        Self {
            name: name.into(),
            system_package,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn system_package(&self) -> &SystemPackage {
        &self.system_package
    }
}
