use super::external_package::ExternalPackage;
use super::modules_builder::ModulesBuilder;
use super::modules_linker::ModulesLinker;
use super::package_interface::PackageInterface;
use crate::common::FilePath;
use crate::common::PackageConfiguration;
use crate::infra::Logger;
use std::collections::HashMap;

pub struct PackageBuilder<'a> {
    modules_builder: &'a ModulesBuilder<'a>,
    modules_linker: &'a ModulesLinker<'a>,
    logger: &'a dyn Logger,
}

impl<'a> PackageBuilder<'a> {
    pub fn new(
        modules_builder: &'a ModulesBuilder<'a>,
        modules_linker: &'a ModulesLinker<'a>,
        logger: &'a dyn Logger,
    ) -> Self {
        Self {
            modules_builder,
            modules_linker,
            logger,
        }
    }

    pub fn build(
        &self,
        package_configuration: &PackageConfiguration,
        external_module_interfaces: &HashMap<
            ExternalPackage,
            HashMap<ein::ExternalUnresolvedModulePath, ein::ModuleInterface>,
        >,
        prelude_package_interface: Option<&PackageInterface>,
    ) -> Result<(FilePath, FilePath), Box<dyn std::error::Error>> {
        self.logger.log(&format!(
            "building package {} {}",
            package_configuration.package().name(),
            package_configuration.package().version()
        ))?;

        let external_module_interfaces = package_configuration
            .build_configuration()
            .dependencies()
            .iter()
            .map(|(name, configuration)| {
                external_module_interfaces[&ExternalPackage::new(name, configuration.version())]
                    .iter()
                    .map(|(module_path, module_interface)| {
                        (module_path.clone(), module_interface.clone())
                    })
            })
            .flatten()
            .collect();

        let (object_file_paths, interface_file_paths) = self.modules_builder.build(
            &package_configuration,
            &external_module_interfaces,
            prelude_package_interface,
        )?;

        self.modules_linker.link(
            &object_file_paths,
            &interface_file_paths,
            package_configuration.directory_path(),
        )
    }
}
