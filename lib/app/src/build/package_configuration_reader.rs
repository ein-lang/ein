use crate::common::PackageConfiguration;
use crate::common::{FilePath, FilePathConfiguration};
use crate::infra::{FilePathDisplayer, FileSystem};

pub struct PackageConfigurationReader<'a> {
    file_system: &'a dyn FileSystem,
    file_path_displayer: &'a dyn FilePathDisplayer,
    file_path_configuration: &'a FilePathConfiguration,
}

impl<'a> PackageConfigurationReader<'a> {
    pub fn new(
        file_system: &'a dyn FileSystem,
        file_path_displayer: &'a dyn FilePathDisplayer,
        file_path_configuration: &'a FilePathConfiguration,
    ) -> Self {
        Self {
            file_system,
            file_path_displayer,
            file_path_configuration,
        }
    }

    pub fn read(
        &self,
        directory_path: &FilePath,
    ) -> Result<PackageConfiguration, Box<dyn std::error::Error>> {
        let repository = self.file_system.read_repository(directory_path)?;

        Ok(PackageConfiguration::new(
            if let Some(repository) = repository {
                // Normalize paths.
                let path = repository
                    .url()
                    .path()
                    .split('/')
                    .filter(|component| component != &"")
                    .collect::<Vec<_>>()
                    .join("/");

                ein::Package::new(
                    [repository.url().host_str().unwrap_or(""), &path].join("/"),
                    repository.version(),
                )
            } else {
                ein::Package::new(self.file_path_displayer.display(directory_path), "")
            },
            serde_json::from_str(&self.file_system.read_to_string(
                &directory_path.join(&self.file_path_configuration.build_configuration_file_path()),
            )?)?,
            directory_path.clone(),
        ))
    }
}
