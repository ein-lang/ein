use super::external_package::ExternalPackage;
use super::package_configuration::PackageConfiguration;
use super::package_initializer::PackageInitializer;
use super::path::FilePathManager;
use crate::infra::{ExternalPackageDownloader, FileStorage};
use std::collections::HashMap;

pub struct ExternalPackagesDownloader<'a> {
    package_initializer: &'a PackageInitializer<'a>,
    external_package_downloader: &'a dyn ExternalPackageDownloader,
    file_storage: &'a dyn FileStorage,
    file_path_manager: &'a FilePathManager<'a>,
}

impl<'a> ExternalPackagesDownloader<'a> {
    pub fn new(
        package_initializer: &'a PackageInitializer<'a>,
        external_package_downloader: &'a dyn ExternalPackageDownloader,
        file_storage: &'a dyn FileStorage,
        file_path_manager: &'a FilePathManager<'a>,
    ) -> Self {
        Self {
            package_initializer,
            external_package_downloader,
            file_storage,
            file_path_manager,
        }
    }

    pub fn download(
        &self,
        main_package_configuration: &PackageConfiguration,
    ) -> Result<HashMap<ExternalPackage, PackageConfiguration>, Box<dyn std::error::Error>> {
        let mut package_configurations = HashMap::new();

        for (name, configuration) in main_package_configuration
            .build_configuration()
            .dependencies()
        {
            let external_package = ExternalPackage::new(name, configuration.version());
            let directory_path = self
                .file_path_manager
                .resolve_to_external_package_directory_path(&external_package);

            if !self.file_storage.exists(&directory_path) {
                self.external_package_downloader
                    .download(&external_package, &directory_path)?;
            }

            let package_configuration = self.package_initializer.initialize(&directory_path)?;

            package_configurations.extend(self.download(&package_configuration)?);
            package_configurations.insert(external_package, package_configuration);
        }

        Ok(package_configurations)
    }
}
