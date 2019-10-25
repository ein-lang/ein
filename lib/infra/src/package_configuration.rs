use super::infrastructure_error::InfrastructureError;
use super::raw_package_configuration::RawPackageConfiguration;
use std::path::{Path, PathBuf};

pub struct PackageConfiguration {
    package_directory: Box<Path>,
    raw: RawPackageConfiguration,
}

impl PackageConfiguration {
    pub fn read(path: impl AsRef<Path>) -> Result<Self, InfrastructureError> {
        Ok(Self {
            package_directory: path
                .as_ref()
                .canonicalize()?
                .parent()
                .ok_or(InfrastructureError::NoParentDirectory)?
                .into(),
            raw: serde_json::from_str(&std::fs::read_to_string(path)?)?,
        })
    }

    pub fn name(&self) -> &str {
        self.raw.name()
    }

    pub fn version(&self) -> &str {
        self.raw.version()
    }

    pub fn source_directory(&self) -> PathBuf {
        self.package_directory.join("src")
    }
}
