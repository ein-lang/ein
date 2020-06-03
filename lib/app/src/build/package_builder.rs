use super::external_package::ExternalPackage;
use super::modules_builder::ModulesBuilder;
use super::package_configuration::PackageConfiguration;
use super::modules_linker::ModulesLinker;
use crate::infra::FilePath;
use std::collections::HashMap;

pub struct PackageBuilder<'a> {
    modules_builder: &'a ModulesBuilder<'a>,
    modules_linker: &'a ModulesLinker<'a>,
}

impl<'a> PackageBuilder<'a> {
    pub fn new(
        modules_builder: &'a ModulesBuilder<'a>,
        modules_linker: &'a ModulesLinker<'a>,
    ) -> Self {
        Self {
            modules_builder,
            modules_linker,
        }
    }

    pub fn build(
        &self,
        package_configuration: &PackageConfiguration,
        external_module_interfaces: &HashMap<
            ExternalPackage,
            HashMap<ein::ExternalUnresolvedModulePath, ein::ModuleInterface>,
        >,
    ) -> Result<(FilePath, Vec<FilePath>), Box<dyn std::error::Error>> {
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

        let (object_file_paths, interface_file_paths) = self
            .modules_builder
            .build(&package_configuration, &external_module_interfaces)?;

        Ok((
            self.modules_linker.link(
                &object_file_paths,
                &interface_file_paths,
                package_configuration.directory_path(),
            )?,
            interface_file_paths,
        ))
    }
}
