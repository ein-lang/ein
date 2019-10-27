use super::raw_package_configuration::RawPackageConfiguration;

pub struct PackageConfiguration {
    name: String,
    version: semver::Version,
}

impl PackageConfiguration {
    pub fn from_json(source: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let configuration: RawPackageConfiguration = serde_json::from_str(source)?;

        Ok(Self {
            name: configuration.name().into(),
            version: semver::Version::parse(configuration.version())?,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &semver::Version {
        &self.version
    }
}
