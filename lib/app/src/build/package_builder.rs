use super::external_package_initializer::ExternalPackageInitializer;
use super::module_builder::ModuleBuilder;
use super::package_configuration::Target;
use super::package_initializer::PackageInitializer;
use super::package_linker::PackageLinker;
use super::path::FilePathManager;
use crate::infra::{
    CommandLinker, ExternalPackageBuilder, ExternalPackageDownloader, FileStorage, LibraryArchiver,
    ObjectLinker, Repository,
};

pub struct PackageBuilder<
    'a,
    R: Repository,
    S: FileStorage,
    OL: ObjectLinker,
    CL: CommandLinker,
    A: LibraryArchiver,
    D: ExternalPackageDownloader,
    B: ExternalPackageBuilder,
> {
    module_builder: &'a ModuleBuilder<'a, S>,
    package_linker: &'a PackageLinker<'a, S, OL>,
    archiver: &'a A,
    command_linker: &'a CL,
    file_path_manager: &'a FilePathManager<'a>,
    package_initializer: &'a PackageInitializer<'a, R, S>,
    external_package_initializer: &'a ExternalPackageInitializer<'a, S, D, B>,
}

impl<
        'a,
        R: Repository,
        S: FileStorage,
        OL: ObjectLinker,
        CL: CommandLinker,
        A: LibraryArchiver,
        D: ExternalPackageDownloader,
        B: ExternalPackageBuilder,
    > PackageBuilder<'a, R, S, OL, CL, A, D, B>
{
    pub fn new(
        module_builder: &'a ModuleBuilder<'a, S>,
        package_linker: &'a PackageLinker<'a, S, OL>,
        archiver: &'a A,
        command_linker: &'a CL,
        file_path_manager: &'a FilePathManager<'a>,
        package_initializer: &'a PackageInitializer<'a, R, S>,
        external_package_initializer: &'a ExternalPackageInitializer<'a, S, D, B>,
    ) -> Self {
        Self {
            module_builder,
            package_linker,
            archiver,
            command_linker,
            file_path_manager,
            package_initializer,
            external_package_initializer,
        }
    }

    pub fn build(&self) -> Result<(), Box<dyn std::error::Error>> {
        let (package, package_configuration) = self.package_initializer.initialize()?;

        let (external_package_object_file_paths, external_module_interfaces) = self
            .external_package_initializer
            .initialize(&package_configuration)?;

        let (object_file_paths, interface_file_paths) = self
            .module_builder
            .build(&package, &external_module_interfaces)?;

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
                self.archiver.archive(
                    &package_object_file_path,
                    self.file_path_manager.archive_package_object_file_path(),
                    &package_interface_file_path,
                    self.file_path_manager.archive_package_interface_file_path(),
                )?;
            }
        }

        Ok(())
    }
}
