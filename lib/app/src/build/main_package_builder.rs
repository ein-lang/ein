use super::external_packages_builder::ExternalPackagesBuilder;
use super::external_packages_downloader::ExternalPackagesDownloader;
use super::package_builder::PackageBuilder;
use super::package_configuration::Target;
use super::package_initializer::PackageInitializer;
use crate::infra::{CommandLinker, FilePath};

pub struct MainPackageBuilder<'a> {
    package_initializer: &'a PackageInitializer<'a>,
    package_builder: &'a PackageBuilder<'a>,
    command_linker: &'a dyn CommandLinker,
    external_packages_downloader: &'a ExternalPackagesDownloader<'a>,
    external_packages_builder: &'a ExternalPackagesBuilder<'a>,
}

impl<'a> MainPackageBuilder<'a> {
    pub fn new(
        package_initializer: &'a PackageInitializer<'a>,
        package_builder: &'a PackageBuilder<'a>,
        command_linker: &'a dyn CommandLinker,
        external_packages_downloader: &'a ExternalPackagesDownloader<'a>,
        external_packages_builder: &'a ExternalPackagesBuilder<'a>,
    ) -> Self {
        Self {
            package_initializer,
            package_builder,
            command_linker,
            external_packages_downloader,
            external_packages_builder,
        }
    }

    pub fn build(&self) -> Result<(), Box<dyn std::error::Error>> {
        let package_configuration = self.package_initializer.initialize(&FilePath::empty())?;

        let external_package_configurations = self
            .external_packages_downloader
            .download(&package_configuration)?;

        let (external_package_object_file_paths, external_module_interfaces) = self
            .external_packages_builder
            .build(&external_package_configurations)?;

        let (package_object_file_path, _) = self
            .package_builder
            .build(&package_configuration, &external_module_interfaces)?;

        match package_configuration.build_configuration().target() {
            Target::Command(command_target) => {
                self.command_linker.link(
                    &vec![package_object_file_path]
                        .into_iter()
                        .chain(external_package_object_file_paths)
                        .collect::<Vec<_>>(),
                    command_target.name(),
                )?;
            }
            Target::Library => {}
        }

        Ok(())
    }
}
