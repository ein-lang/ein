use super::external_packages_builder::ExternalPackagesBuilder;
use super::external_packages_downloader::ExternalPackagesDownloader;
use super::package_builder::PackageBuilder;
use super::package_configuration_reader::PackageConfigurationReader;
use super::prelude_package_builder::PreludePackageBuilder;
use crate::common::{CommandTarget, FilePath, PackageConfiguration, Target};
use crate::infra::{CommandLinker, Logger};

pub struct MainPackageBuilder<'a> {
    package_configuration_reader: &'a PackageConfigurationReader<'a>,
    package_builder: &'a PackageBuilder<'a>,
    command_linker: &'a dyn CommandLinker,
    prelude_package_builder: &'a PreludePackageBuilder<'a>,
    external_packages_downloader: &'a ExternalPackagesDownloader<'a>,
    external_packages_builder: &'a ExternalPackagesBuilder<'a>,
    logger: &'a dyn Logger,
}

impl<'a> MainPackageBuilder<'a> {
    pub fn new(
        package_configuration_reader: &'a PackageConfigurationReader<'a>,
        package_builder: &'a PackageBuilder<'a>,
        command_linker: &'a dyn CommandLinker,
        prelude_package_builder: &'a PreludePackageBuilder<'a>,
        external_packages_downloader: &'a ExternalPackagesDownloader<'a>,
        external_packages_builder: &'a ExternalPackagesBuilder<'a>,
        logger: &'a dyn Logger,
    ) -> Self {
        Self {
            package_configuration_reader,
            package_builder,
            command_linker,
            prelude_package_builder,
            external_packages_downloader,
            external_packages_builder,
            logger,
        }
    }

    pub fn build(&self) -> Result<(), Box<dyn std::error::Error>> {
        let package_configuration = self.package_configuration_reader.read(&FilePath::empty())?;

        match package_configuration.build_configuration().target() {
            Target::Command(command_target) => {
                self.build_command(&package_configuration, command_target)
            }
            Target::Library => self.build_library(&package_configuration),
        }
    }

    fn build_command(
        &self,
        package_configuration: &PackageConfiguration,
        command_target: &CommandTarget,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (prelude_package_object_file_paths, prelude_module_interfaces) =
            self.prelude_package_builder.build()?;

        let external_package_configurations = self
            .external_packages_downloader
            .download(&package_configuration)?;

        let (external_module_object_file_paths, external_module_interfaces) = self
            .external_packages_builder
            .build(&external_package_configurations, &prelude_module_interfaces)?;

        let (module_object_file_paths, _) = self.package_builder.build(
            &package_configuration,
            &external_module_interfaces,
            &prelude_module_interfaces,
        )?;

        self.logger
            .log(&format!("linking command {}", command_target.name()))?;

        self.command_linker.link(
            &prelude_package_object_file_paths
                .into_iter()
                .chain(external_module_object_file_paths)
                .chain(module_object_file_paths)
                .collect::<Vec<_>>(),
            command_target.name(),
        )?;

        Ok(())
    }

    fn build_library(
        &self,
        package_configuration: &PackageConfiguration,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (_, prelude_module_interfaces) = self.prelude_package_builder.build()?;

        let external_package_configurations = self
            .external_packages_downloader
            .download(&package_configuration)?;

        let (_, external_module_interfaces) = self
            .external_packages_builder
            .build(&external_package_configurations, &prelude_module_interfaces)?;

        self.package_builder.build(
            &package_configuration,
            &external_module_interfaces,
            &prelude_module_interfaces,
        )?;

        Ok(())
    }
}
