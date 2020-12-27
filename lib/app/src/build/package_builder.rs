use super::modules_builder::ModulesBuilder;
use super::modules_linker::ModulesLinker;
use super::package_interface::PackageInterface;
use crate::common::{ExternalPackage, FilePath, PackageConfiguration};
use crate::infra::{FfiPackageInitializer, Logger};
use std::collections::HashMap;

pub struct PackageBuilder<'a> {
    modules_builder: &'a ModulesBuilder<'a>,
    modules_linker: &'a ModulesLinker<'a>,
    logger: &'a dyn Logger,
    ffi_package_initializer: &'a dyn FfiPackageInitializer,
}

impl<'a> PackageBuilder<'a> {
    pub fn new(
        modules_builder: &'a ModulesBuilder<'a>,
        modules_linker: &'a ModulesLinker<'a>,
        logger: &'a dyn Logger,
        ffi_package_initializer: &'a dyn FfiPackageInitializer,
    ) -> Self {
        Self {
            modules_builder,
            modules_linker,
            logger,
            ffi_package_initializer,
        }
    }

    pub fn build(
        &self,
        package_configuration: &PackageConfiguration,
        external_module_interfaces: &HashMap<
            ExternalPackage,
            HashMap<lang::ExternalUnresolvedModulePath, lang::ModuleInterface>,
        >,
        prelude_package_interface: Option<&PackageInterface>,
    ) -> Result<(Vec<FilePath>, FilePath), Box<dyn std::error::Error>> {
        let ffi_object_file_path = self
            .ffi_package_initializer
            .initialize(package_configuration.directory_path())?;

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

        let (package_object_file_path, package_interface_file_path) = self.modules_linker.link(
            &object_file_paths,
            &interface_file_paths,
            package_configuration.directory_path(),
        )?;

        Ok((
            vec![package_object_file_path]
                .into_iter()
                .chain(ffi_object_file_path)
                .collect(),
            package_interface_file_path,
        ))
    }
}
