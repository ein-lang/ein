pub struct PackageConfiguration {
    name: String,
    version: semver::Version,
}

impl PackageConfiguration {
    pub fn new(name: String, version: semver::Version) -> Self {
        Self { name, version }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &semver::Version {
        &self.version
    }
}
