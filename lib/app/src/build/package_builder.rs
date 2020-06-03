use super::external_package_id::ExternalPackageId;
use super::module_builder::ModuleBuilder;
use super::package_initializer::PackageInitializer;
use super::package_linker::PackageLinker;
use crate::infra::FilePath;
use std::collections::HashMap;

pub struct PackageBuilder<'a> {
    module_builder: &'a ModuleBuilder<'a>,
    package_linker: &'a PackageLinker<'a>,
    package_initializer: &'a PackageInitializer<'a>,
}

impl<'a> PackageBuilder<'a> {
    pub fn new(
        module_builder: &'a ModuleBuilder<'a>,
        package_linker: &'a PackageLinker<'a>,
        package_initializer: &'a PackageInitializer<'a>,
    ) -> Self {
        Self {
            module_builder,
            package_linker,
            package_initializer,
        }
    }

    pub fn build(
        &self,
        directory_path: &FilePath,
        external_module_interfaces: &HashMap<
            ExternalPackageId,
            HashMap<ein::ExternalUnresolvedModulePath, ein::ModuleInterface>,
        >,
    ) -> Result<(FilePath, Vec<FilePath>), Box<dyn std::error::Error>> {
        let package_configuration = self.package_initializer.initialize(directory_path)?;

        let external_module_interfaces = package_configuration
            .build_configuration()
            .dependencies()
            .iter()
            .map(|(name, configuration)| {
                external_module_interfaces[&ExternalPackageId::new(name, configuration.version())]
                    .iter()
                    .map(|(module_path, module_interface)| {
                        (module_path.clone(), module_interface.clone())
                    })
            })
            .flatten()
            .collect();

        let (object_file_paths, interface_file_paths) = self
            .module_builder
            .build(&package_configuration, &external_module_interfaces)?;

        Ok((
            self.package_linker
                .link(&object_file_paths, &interface_file_paths, &directory_path)?,
            interface_file_paths,
        ))
    }
}
