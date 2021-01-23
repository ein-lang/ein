use super::modules_builder::ModulesBuilder;
use crate::common::{ExternalPackage, FilePath, PackageConfiguration};
use crate::infra::{FfiPackageInitializer, FileSystem, Logger};
use std::collections::HashMap;

pub struct PackageBuilder<'a> {
    modules_builder: &'a ModulesBuilder<'a>,
    ffi_package_initializer: &'a dyn FfiPackageInitializer,
    file_system: &'a dyn FileSystem,
    logger: &'a dyn Logger,
}

impl<'a> PackageBuilder<'a> {
    pub fn new(
        modules_builder: &'a ModulesBuilder<'a>,
        ffi_package_initializer: &'a dyn FfiPackageInitializer,
        file_system: &'a dyn FileSystem,
        logger: &'a dyn Logger,
    ) -> Self {
        Self {
            modules_builder,
            ffi_package_initializer,
            file_system,
            logger,
        }
    }

    pub fn build(
        &self,
        package_configuration: &PackageConfiguration,
        external_module_interfaces: &HashMap<
            ExternalPackage,
            HashMap<lang::ExternalUnresolvedModulePath, lang::ModuleInterface>,
        >,
        prelude_module_interfaces: &[lang::ModuleInterface],
    ) -> Result<(Vec<FilePath>, Vec<lang::ModuleInterface>), Box<dyn std::error::Error>> {
        self.logger.log(&format!(
            "building package {} {}",
            package_configuration.package().name(),
            package_configuration.package().version()
        ))?;

        let ffi_object_file_path = if self
            .ffi_package_initializer
            .is_ffi_used(package_configuration.directory_path())
        {
            self.logger.log(&format!(
                "building FFI for package {} {}",
                package_configuration.package().name(),
                package_configuration.package().version()
            ))?;

            self.ffi_package_initializer
                .initialize(package_configuration.directory_path())?
        } else {
            None
        };

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
            prelude_module_interfaces,
        )?;

        Ok((
            object_file_paths
                .into_iter()
                .chain(ffi_object_file_path)
                .collect(),
            interface_file_paths
                .iter()
                .map(|file_path| {
                    Ok(serde_json::from_slice(
                        &self.file_system.read_to_vec(&file_path)?,
                    )?)
                })
                .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?,
        ))
    }
}
