use super::infrastructure_error::InfrastructureError;
use super::raw_package_configuration::RawPackageConfiguration;
use std::path::{Path, PathBuf};

pub struct PackageConfiguration {
    name: String,
    package_directory: Box<Path>,
    version: semver::Version,
}

impl PackageConfiguration {
    pub fn read(path: impl AsRef<Path>) -> Result<Self, InfrastructureError> {
        let configuration: RawPackageConfiguration =
            serde_json::from_str(&std::fs::read_to_string(&path)?)?;

        Ok(Self {
            name: configuration.name().into(),
            package_directory: path
                .as_ref()
                .canonicalize()?
                .parent()
                .ok_or(InfrastructureError::NoParentDirectory)?
                .into(),
            version: semver::Version::parse(configuration.version())?,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &semver::Version {
        &self.version
    }

    pub fn source_directory(&self) -> PathBuf {
        self.package_directory.join("src")
    }
}
