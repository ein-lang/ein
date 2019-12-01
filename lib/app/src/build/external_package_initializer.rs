use super::package_configuration::PackageConfiguration;
use super::path::ExternalModulePathManager;
use crate::infra::{ExternalPackageBuilder, ExternalPackageDownloader, FileStorage};

pub struct ExternalPackageInitializer<
    'a,
    S: FileStorage,
    D: ExternalPackageDownloader,
    B: ExternalPackageBuilder,
> {
    external_package_downloader: &'a D,
    external_package_builder: &'a B,
    external_module_path_manager: &'a ExternalModulePathManager<'a>,
    file_storage: &'a S,
}

impl<'a, S: FileStorage, D: ExternalPackageDownloader, B: ExternalPackageBuilder>
    ExternalPackageInitializer<'a, S, D, B>
{
    pub fn new(
        external_package_downloader: &'a D,
        external_package_builder: &'a B,
        external_module_path_manager: &'a ExternalModulePathManager<'a>,
        file_storage: &'a S,
    ) -> Self {
        Self {
            external_package_downloader,
            external_package_builder,
            external_module_path_manager,
            file_storage,
        }
    }

    pub fn initialize(
        &self,
        package_configuration: &PackageConfiguration,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (name, external_package) in package_configuration.dependencies() {
            let directory_path = self
                .external_module_path_manager
                .convert_to_directory_path(name);

            if self.file_storage.exists(&directory_path) {
                continue;
            }

            self.external_package_downloader.download(
                name,
                external_package.version(),
                &directory_path,
            )?;

            self.external_package_builder.build(&directory_path)?;
        }

        Ok(())
    }
}
