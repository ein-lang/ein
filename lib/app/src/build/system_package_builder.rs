use super::external_packages_downloader::ExternalPackagesDownloader;
use super::package_builder::PackageBuilder;
use crate::common::{ExternalPackage, FilePath, SystemPackageConfiguration};

pub struct SystemPackageBuilder<'a> {
    package_builder: &'a PackageBuilder<'a>,
    external_packages_downloader: &'a ExternalPackagesDownloader<'a>,
}

impl<'a> SystemPackageBuilder<'a> {
    pub fn new(
        package_builder: &'a PackageBuilder<'a>,
        external_packages_downloader: &'a ExternalPackagesDownloader<'a>,
    ) -> Self {
        Self {
            package_builder,
            external_packages_downloader,
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

        let package_configuration = self
            .external_packages_downloader
            .download_one(&external_package)?;

        self.package_builder.build(
            &package_configuration,
            &Default::default(),
            prelude_module_interfaces,
        )
    }
}
