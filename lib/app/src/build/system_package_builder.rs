use super::cached_external_package_downloader::CachedExternalPackageDownloader;
use super::package_builder::PackageBuilder;
use crate::common::{ExternalPackage, FilePath};

pub struct SystemPackageBuilder<'a> {
    package_builder: &'a PackageBuilder<'a>,
    cached_external_package_downloader: &'a CachedExternalPackageDownloader<'a>,
}

impl<'a> SystemPackageBuilder<'a> {
    pub fn new(
        package_builder: &'a PackageBuilder<'a>,
        cached_external_package_downloader: &'a CachedExternalPackageDownloader<'a>,
    ) -> Self {
        Self {
            package_builder,
            cached_external_package_downloader,
        }
    }

    pub fn build(
        &self,
        system_package: &ExternalPackage,
        prelude_module_interfaces: &[lang::ModuleInterface],
    ) -> Result<(Vec<FilePath>, Vec<lang::ModuleInterface>), Box<dyn std::error::Error>> {
        let package_configuration =
            self.cached_external_package_downloader
                .download(&ExternalPackage::new(
                    system_package.name(),
                    system_package.version(),
                ))?;

        self.package_builder.build(
            &package_configuration,
            &Default::default(),
            prelude_module_interfaces,
        )
    }
}
