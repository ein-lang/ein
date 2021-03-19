use super::external_packages_builder::ExternalPackagesBuilder;
use super::external_packages_downloader::ExternalPackagesDownloader;
use super::package_builder::PackageBuilder;
use super::package_configuration_reader::PackageConfigurationReader;
use super::prelude_package_builder::PreludePackageBuilder;
use super::system_package_builder::SystemPackageBuilder;
use super::utilities::convert_module_interface_vec_to_map;
use crate::common::{ApplicationTarget, FilePath, PackageConfiguration, Target};
use crate::infra::{ApplicationLinker, Logger};
use std::collections::HashMap;

pub struct MainPackageBuilder<'a> {
    package_configuration_reader: &'a PackageConfigurationReader<'a>,
    package_builder: &'a PackageBuilder<'a>,
    application_linker: &'a dyn ApplicationLinker,
    prelude_package_builder: &'a PreludePackageBuilder<'a>,
    system_package_builder: &'a SystemPackageBuilder<'a>,
    external_packages_downloader: &'a ExternalPackagesDownloader<'a>,
    external_packages_builder: &'a ExternalPackagesBuilder<'a>,
    logger: &'a dyn Logger,
}

impl<'a> MainPackageBuilder<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        package_configuration_reader: &'a PackageConfigurationReader<'a>,
        package_builder: &'a PackageBuilder<'a>,
        application_linker: &'a dyn ApplicationLinker,
        prelude_package_builder: &'a PreludePackageBuilder<'a>,
        system_package_builder: &'a SystemPackageBuilder<'a>,
        external_packages_downloader: &'a ExternalPackagesDownloader<'a>,
        external_packages_builder: &'a ExternalPackagesBuilder<'a>,
        logger: &'a dyn Logger,
    ) -> Self {
        Self {
            package_configuration_reader,
            package_builder,
            application_linker,
            prelude_package_builder,
            system_package_builder,
            external_packages_downloader,
            external_packages_builder,
            logger,
        }
    }

    pub fn build(&self) -> Result<(), Box<dyn std::error::Error>> {
        let package_configuration = self.package_configuration_reader.read(&FilePath::empty())?;

        match package_configuration.build_configuration().target() {
            Target::Application(application_target) => {
                self.build_application(&package_configuration, application_target)
            }
            Target::Library => self.build_library(&package_configuration),
        }
    }

    fn build_application(
        &self,
        package_configuration: &PackageConfiguration,
        application_target: &ApplicationTarget,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (prelude_module_object_paths, prelude_module_interfaces) =
            self.prelude_package_builder.build()?;

        let (system_module_object_paths, system_module_interfaces) =
            self.system_package_builder.build(
                application_target.system_package(),
                &prelude_module_interfaces,
            )?;

        // TODO Combine only the MainFunction module.
        let prelude_module_interfaces = prelude_module_interfaces
            .into_iter()
            .chain(system_module_interfaces.clone())
            .collect::<Vec<_>>();

        let external_package_configurations = self.external_packages_downloader.download(
            &package_configuration
                .build_configuration()
                .dependencies()
                .iter()
                .collect::<Vec<_>>(),
        )?;

        let (external_module_object_paths, mut external_module_interfaces) = self
            .external_packages_builder
            .build(&external_package_configurations, &prelude_module_interfaces)?;

        let (module_object_paths, _) = self.package_builder.build(
            &package_configuration,
            &external_module_interfaces
                .drain()
                .chain(vec![(
                    application_target.system_package().clone(),
                    convert_module_interface_vec_to_map(&system_module_interfaces),
                )])
                .collect::<HashMap<_, _>>(),
            &prelude_module_interfaces,
        )?;

        self.logger.log(&format!(
            "linking application {}",
            application_target.name()
        ))?;

        self.application_linker.link(
            &prelude_module_object_paths
                .into_iter()
                .chain(system_module_object_paths)
                .chain(external_module_object_paths)
                .chain(module_object_paths)
                .collect::<Vec<_>>(),
            application_target.name(),
        )?;

        Ok(())
    }

    fn build_library(
        &self,
        package_configuration: &PackageConfiguration,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (_, prelude_module_interfaces) = self.prelude_package_builder.build()?;

        let external_package_configurations = self.external_packages_downloader.download(
            &package_configuration
                .build_configuration()
                .dependencies()
                .iter()
                .collect::<Vec<_>>(),
        )?;

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
