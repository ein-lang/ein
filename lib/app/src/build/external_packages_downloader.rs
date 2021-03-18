use super::cached_external_package_downloader::CachedExternalPackageDownloader;
use crate::common::{ExternalPackage, PackageConfiguration};
use std::collections::HashMap;

pub struct ExternalPackagesDownloader<'a> {
    cached_external_package_downloader: &'a CachedExternalPackageDownloader<'a>,
}

impl<'a> ExternalPackagesDownloader<'a> {
    pub fn new(
        cached_external_package_downloader: &'a CachedExternalPackageDownloader<'a>,
    ) -> Self {
        Self {
            cached_external_package_downloader,
        }
    }

    pub fn download(
        &self,
        external_packages: &[&ExternalPackage],
    ) -> Result<HashMap<ExternalPackage, PackageConfiguration>, Box<dyn std::error::Error>> {
        let mut package_configurations = HashMap::new();

        for &external_package in external_packages {
            let package_configuration = self
                .cached_external_package_downloader
                .download(&external_package)?;

            package_configurations.extend(
                self.download(
                    &package_configuration
                        .build_configuration()
                        .dependencies()
                        .into_iter()
                        .collect::<Vec<_>>(),
                )?,
            );

            package_configurations.insert(external_package.clone(), package_configuration);
        }

        Ok(package_configurations)
    }
}
