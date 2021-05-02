use crate::{
    adaptors::deserialize_build_configuration,
    common::{FilePath, PackageConfiguration, StaticFilePathManager},
    infra::{FilePathDisplayer, FileSystem},
};

pub struct PackageConfigurationReader<'a> {
    file_system: &'a dyn FileSystem,
    file_path_displayer: &'a dyn FilePathDisplayer,
    static_file_path_manager: &'a StaticFilePathManager,
}

impl<'a> PackageConfigurationReader<'a> {
    pub fn new(
        file_system: &'a dyn FileSystem,
        file_path_displayer: &'a dyn FilePathDisplayer,
        static_file_path_manager: &'a StaticFilePathManager,
    ) -> Self {
        Self {
            file_system,
            file_path_displayer,
            static_file_path_manager,
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

                lang::Package::new(
                    [repository.url().host_str().unwrap_or(""), &path].join("/"),
                    repository.version(),
                )
            } else {
                lang::Package::new(self.file_path_displayer.display(directory_path), "")
            },
            deserialize_build_configuration(
                &self.file_system.read_to_string(
                    &directory_path.join(
                        &self
                            .static_file_path_manager
                            .build_configuration_file_path(),
                    ),
                )?,
            )?,
            directory_path.clone(),
        ))
    }
}
