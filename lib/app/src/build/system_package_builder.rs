use super::package_builder::PackageBuilder;
use super::package_configuration_reader::PackageConfigurationReader;
use crate::common::{ExternalPackage, FilePath, FilePathResolver, SystemPackageConfiguration};
use crate::infra::ExternalPackageDownloader;

pub struct SystemPackageBuilder<'a> {
    package_configuration_reader: &'a PackageConfigurationReader<'a>,
    package_builder: &'a PackageBuilder<'a>,
    file_path_resolver: &'a FilePathResolver<'a>,
    external_package_downloader: &'a dyn ExternalPackageDownloader,
}

impl<'a> SystemPackageBuilder<'a> {
    pub fn new(
        package_configuration_reader: &'a PackageConfigurationReader<'a>,
        package_builder: &'a PackageBuilder<'a>,
        file_path_resolver: &'a FilePathResolver<'a>,
        external_package_downloader: &'a dyn ExternalPackageDownloader,
    ) -> Self {
        Self {
            package_configuration_reader,
            package_builder,
            file_path_resolver,
            external_package_downloader,
        }
    }

    pub fn build(
        &self,
        system_package_configuration: &SystemPackageConfiguration,
        prelude_module_interfaces: &[lang::ModuleInterface],
    ) -> Result<(Vec<FilePath>, Vec<lang::ModuleInterface>), Box<dyn std::error::Error>> {
        let external_package = ExternalPackage::new(
            system_package_configuration.name(),
            system_package_configuration.version(),
        );

        let directory_path = self
            .file_path_resolver
            .resolve_external_package_directory_path(&external_package);

        self.external_package_downloader
            .download(&external_package, &directory_path)?;

        let package_configuration = self.package_configuration_reader.read(&directory_path)?;

        self.package_builder.build(
            &package_configuration,
            &Default::default(),
            prelude_module_interfaces,
        )
    }
}
