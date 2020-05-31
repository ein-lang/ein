use super::package_configuration::PackageConfiguration;
use super::path::FilePathConfiguration;
use crate::infra::{FilePath, FileStorage, Repository};

pub struct PackageInitializer<'a> {
    repository: &'a dyn Repository,
    file_storage: &'a dyn FileStorage,
    file_path_configuration: &'a FilePathConfiguration,
}

impl<'a> PackageInitializer<'a> {
    pub fn new(
        repository: &'a dyn Repository,
        file_storage: &'a dyn FileStorage,
        file_path_configuration: &'a FilePathConfiguration,
    ) -> Self {
        Self {
            repository,
            file_storage,
            file_path_configuration,
        }
    }

    pub fn initialize(
        &self,
    ) -> Result<(ein::Package, PackageConfiguration), Box<dyn std::error::Error>> {
        Ok((
            self.repository.get_package()?,
            serde_json::from_str(
                &self.file_storage.read_to_string(&FilePath::new(&[self
                    .file_path_configuration
                    .package_configuration_filename()]))?,
            )?,
        ))
    }
}
