use super::external_package_initializer::ExternalPackageInitializer;
use super::module_builder::ModuleBuilder;
use super::package_configuration::Target;
use super::package_initializer::PackageInitializer;
use super::package_linker::PackageLinker;
use super::path::FilePathManager;
use crate::infra::{CommandLinker, FilePath, LibraryArchiver};

pub struct PackageBuilder<'a> {
    module_builder: &'a ModuleBuilder<'a>,
    package_linker: &'a PackageLinker<'a>,
    library_archiver: &'a dyn LibraryArchiver,
    command_linker: &'a dyn CommandLinker,
    file_path_manager: &'a FilePathManager<'a>,
    package_initializer: &'a PackageInitializer<'a>,
    external_package_initializer: &'a ExternalPackageInitializer<'a>,
}

impl<'a> PackageBuilder<'a> {
    pub fn new(
        module_builder: &'a ModuleBuilder<'a>,
        package_linker: &'a PackageLinker<'a>,
        library_archiver: &'a dyn LibraryArchiver,
        command_linker: &'a dyn CommandLinker,
        file_path_manager: &'a FilePathManager<'a>,
        package_initializer: &'a PackageInitializer<'a>,
        external_package_initializer: &'a ExternalPackageInitializer<'a>,
    ) -> Self {
        Self {
            module_builder,
            package_linker,
            library_archiver,
            command_linker,
            file_path_manager,
            package_initializer,
            external_package_initializer,
        }
    }

    pub fn build(&self) -> Result<(), Box<dyn std::error::Error>> {
        let package_configuration = self.package_initializer.initialize(&FilePath::empty())?;

        let (external_package_object_file_paths, external_module_interfaces) = self
            .external_package_initializer
            .initialize(&package_configuration)?;

        let (object_file_paths, interface_file_paths) = self
            .module_builder
            .build(&package_configuration, &external_module_interfaces)?;

        let (package_object_file_path, package_interface_file_path) = self.package_linker.link(
            &object_file_paths,
            &external_package_object_file_paths,
            &interface_file_paths,
        )?;

        match package_configuration.target() {
            Target::Command(command_target) => {
                self.command_linker
                    .link(&package_object_file_path, command_target.name())?;
            }
            Target::Library => {
                self.library_archiver.archive(
                    &package_object_file_path,
                    self.file_path_manager
                        .configuration()
                        .package_object_file_path(),
                    &package_interface_file_path,
                    self.file_path_manager
                        .configuration()
                        .package_interface_file_path(),
                )?;
            }
        }

        Ok(())
    }
}
