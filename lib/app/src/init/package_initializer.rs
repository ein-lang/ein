use super::package_initialization_configuration::PackageInitializationConfiguration;
use crate::common::{
    BuildConfiguration, FilePath, FilePathConfiguration, StaticFilePathManager, Target,
};
use crate::{adaptors::serialize_build_configuration, infra::FileSystem};

pub struct PackageInitializer<'a> {
    file_system: &'a dyn FileSystem,
    static_file_path_manager: &'a StaticFilePathManager,
    file_path_configuration: &'a FilePathConfiguration,
    package_initialization_configuration: &'a PackageInitializationConfiguration,
}

impl<'a> PackageInitializer<'a> {
    pub fn new(
        file_system: &'a dyn FileSystem,
        static_file_path_manager: &'a StaticFilePathManager,
        file_path_configuration: &'a FilePathConfiguration,
        package_initialization_configuration: &'a PackageInitializationConfiguration,
    ) -> Self {
        Self {
            file_system,
            static_file_path_manager,
            file_path_configuration,
            package_initialization_configuration,
        }
    }

    pub fn initialize(&self, target: &Target) -> Result<(), Box<dyn std::error::Error>> {
        self.file_system.write(
            &FilePath::new(&[self.file_path_configuration.build_configuration_filename]),
            serialize_build_configuration(&BuildConfiguration::new(
                target.clone(),
                Default::default(),
            ))?
            .as_bytes(),
        )?;

        match target {
            Target::Application(_) => {
                self.file_system.write(
                    self.static_file_path_manager.main_source_file_path(),
                    self.package_initialization_configuration
                        .application_main_file_content
                        .as_bytes(),
                )?;
            }
            Target::Library => {
                self.file_system.write(
                    &FilePath::new(&[self
                        .package_initialization_configuration
                        .library_main_basename])
                    .with_extension(&self.file_path_configuration.source_file_extension),
                    self.package_initialization_configuration
                        .library_main_file_content
                        .as_bytes(),
                )?;
            }
        }

        Ok(())
    }
}
