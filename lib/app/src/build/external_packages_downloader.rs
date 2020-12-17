use super::package_configuration_reader::PackageConfigurationReader;
use crate::common::{ExternalPackage, FilePathManager, PackageConfiguration};
use crate::infra::{ExternalPackageDownloader, FileSystem, Logger};
use std::collections::HashMap;

pub struct ExternalPackagesDownloader<'a> {
    package_configuration_reader: &'a PackageConfigurationReader<'a>,
    external_package_downloader: &'a dyn ExternalPackageDownloader,
    file_system: &'a dyn FileSystem,
    file_path_manager: &'a FilePathManager<'a>,
    logger: &'a dyn Logger,
}

impl<'a> ExternalPackagesDownloader<'a> {
    pub fn new(
        package_configuration_reader: &'a PackageConfigurationReader<'a>,
        external_package_downloader: &'a dyn ExternalPackageDownloader,
        file_system: &'a dyn FileSystem,
        file_path_manager: &'a FilePathManager<'a>,
        logger: &'a dyn Logger,
    ) -> Self {
        Self {
            package_configuration_reader,
            external_package_downloader,
            file_system,
            file_path_manager,
            logger,
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

            if !self.file_system.exists(&directory_path) {
                self.logger.log(&format!(
                    "downloading package {} {}",
                    external_package.name(),
                    external_package.version()
                ))?;

                self.external_package_downloader
                    .download(&external_package, &directory_path)?;
            }

            let package_configuration = self.package_configuration_reader.read(&directory_path)?;

            package_configurations.extend(self.download(&package_configuration)?);
            package_configurations.insert(external_package, package_configuration);
        }

        Ok(package_configurations)
    }
}
