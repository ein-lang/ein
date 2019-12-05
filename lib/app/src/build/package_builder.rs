use super::external_package_initializer::ExternalPackageInitializer;
use super::module_builder::ModuleBuilder;
use super::package_configuration::Target;
use super::package_initializer::PackageInitializer;
use super::path::InternalModulePathManager;
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
    object_linker: &'a OL,
    archiver: &'a A,
    command_linker: &'a CL,
    internal_module_path_manager: &'a InternalModulePathManager<'a>,
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
        object_linker: &'a OL,
        archiver: &'a A,
        command_linker: &'a CL,
        internal_module_path_manager: &'a InternalModulePathManager<'a>,
        package_initializer: &'a PackageInitializer<'a, R, S>,
        external_package_initializer: &'a ExternalPackageInitializer<'a, S, D, B>,
    ) -> Self {
        Self {
            module_builder,
            object_linker,
            archiver,
            command_linker,
            internal_module_path_manager,
            package_initializer,
            external_package_initializer,
        }
    }

    pub fn build(&self) -> Result<(), Box<dyn std::error::Error>> {
        let (package, package_configuration) = self.package_initializer.initialize()?;

        self.external_package_initializer
            .initialize(&package_configuration)?;

        let (object_file_paths, interface_file_paths) = self.module_builder.build(&package)?;
        let package_object_file_path = self.internal_module_path_manager.package_object_file_path();
        self.object_linker
            .link(&object_file_paths, &package_object_file_path)?;

        match package_configuration.target() {
            Target::Command(command_target) => {
                self.command_linker
                    .link(&package_object_file_path, command_target.name())?;
            }
            Target::Library => {
                self.archiver.archive(
                    &package_object_file_path,
                    self.internal_module_path_manager
                        .archive_package_object_file_path(),
                    &interface_file_paths
                        .into_iter()
                        .map(|interface_file_path| {
                            (
                                self.internal_module_path_manager
                                    .convert_to_archive_interface_file_path(&interface_file_path),
                                interface_file_path,
                            )
                        })
                        .collect(),
                )?;
            }
        }

        Ok(())
    }
}
