use super::package_builder::PackageBuilder;
use super::package_configuration_reader::PackageConfigurationReader;
use crate::common::{FilePath, StaticFilePathManager};
use crate::infra::PreludePackageDownloader;

pub struct PreludePackageBuilder<'a> {
    package_configuration_reader: &'a PackageConfigurationReader<'a>,
    package_builder: &'a PackageBuilder<'a>,
    prelude_package_downloader: &'a dyn PreludePackageDownloader,
    static_file_path_manager: &'a StaticFilePathManager,
}

impl<'a> PreludePackageBuilder<'a> {
    pub fn new(
        package_configuration_reader: &'a PackageConfigurationReader<'a>,
        package_builder: &'a PackageBuilder<'a>,
        prelude_package_downloader: &'a dyn PreludePackageDownloader,
        static_file_path_manager: &'a StaticFilePathManager,
    ) -> Self {
        Self {
            package_configuration_reader,
            package_builder,
            prelude_package_downloader,
            static_file_path_manager,
        }
    }

    pub fn build(
        &self,
    ) -> Result<(Vec<FilePath>, Vec<lang::ModuleInterface>), Box<dyn std::error::Error>> {
        let directory_path = self
            .static_file_path_manager
            .prelude_package_directory_path();

        self.prelude_package_downloader.download(&directory_path)?;

        let package_configuration = self.package_configuration_reader.read(&directory_path)?;

        let (package_object_file_paths, module_interfaces) =
            self.package_builder
                .build(&package_configuration, &Default::default(), &[])?;

        Ok((package_object_file_paths, module_interfaces))
    }
}
