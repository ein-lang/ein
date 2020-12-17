use super::package_builder::PackageBuilder;
use super::package_configuration_reader::PackageConfigurationReader;
use super::package_interface::PackageInterface;
use crate::common::{FilePath, FilePathManager};
use crate::infra::{FileSystem, PreludePackageDownloader};

pub struct PreludePackageBuilder<'a> {
    package_configuration_reader: &'a PackageConfigurationReader<'a>,
    package_builder: &'a PackageBuilder<'a>,
    prelude_package_downloader: &'a dyn PreludePackageDownloader,
    file_system: &'a dyn FileSystem,
    file_path_manager: &'a FilePathManager<'a>,
}

impl<'a> PreludePackageBuilder<'a> {
    pub fn new(
        package_configuration_reader: &'a PackageConfigurationReader<'a>,
        package_builder: &'a PackageBuilder<'a>,
        prelude_package_downloader: &'a dyn PreludePackageDownloader,
        file_system: &'a dyn FileSystem,
        file_path_manager: &'a FilePathManager<'a>,
    ) -> Self {
        Self {
            package_configuration_reader,
            package_builder,
            prelude_package_downloader,
            file_system,
            file_path_manager,
        }
    }

    pub fn build(&self) -> Result<(FilePath, PackageInterface), Box<dyn std::error::Error>> {
        let directory_path = self
            .file_path_manager
            .configuration()
            .prelude_package_directory_path();

        self.prelude_package_downloader.download(&directory_path)?;

        let package_configuration = self.package_configuration_reader.read(&directory_path)?;

        let (package_object_file_path, package_interface_file_path) =
            self.package_builder
                .build(&package_configuration, &Default::default(), None)?;

        Ok((
            package_object_file_path,
            serde_json::from_str::<PackageInterface>(
                &self
                    .file_system
                    .read_to_string(&package_interface_file_path)?,
            )?,
        ))
    }
}
