use super::package_configuration::PackageConfiguration;
use super::path::FilePathConfiguration;
use crate::infra::{FilePath, FileStorage};

pub struct PackageInitializer<'a> {
    file_storage: &'a dyn FileStorage,
    file_path_configuration: &'a FilePathConfiguration,
}

impl<'a> PackageInitializer<'a> {
    pub fn new(
        file_storage: &'a dyn FileStorage,
        file_path_configuration: &'a FilePathConfiguration,
    ) -> Self {
        Self {
            file_storage,
            file_path_configuration,
        }
    }

    pub fn initialize(
        &self,
        directory_path: &FilePath,
    ) -> Result<PackageConfiguration, Box<dyn std::error::Error>> {
        let repository = self.file_storage.read_repository(directory_path);

        Ok(PackageConfiguration::new(
            repository
                .as_ref()
                .map(|repository| {
                    if let Some(host) = repository.url().host_str() {
                        [host, repository.url().path()].concat()
                    } else {
                        repository.url().path().into()
                    }
                })
                .unwrap_or_else(|_| format!("{}", directory_path)),
            repository
                .map(|repository| String::from(repository.version()))
                .unwrap_or_else(|_| "master".into()),
            serde_json::from_str(
                &self
                    .file_storage
                    .read_to_string(self.file_path_configuration.build_configuration_file_path())?,
            )?,
        ))
    }
}
