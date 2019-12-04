use super::module_builder::ModuleBuilder;
use super::path::InternalModulePathManager;
use crate::infra::{FileStorage, LibraryArchiver, ObjectLinker};

pub struct LibraryPackageBuilder<'a, S: FileStorage, OL: ObjectLinker, A: LibraryArchiver> {
    module_builder: &'a ModuleBuilder<'a, S, OL>,
    archiver: &'a A,
    internal_module_path_manager: &'a InternalModulePathManager<'a>,
}

impl<'a, S: FileStorage, OL: ObjectLinker, A: LibraryArchiver> LibraryPackageBuilder<'a, S, OL, A> {
    pub fn new(
        module_builder: &'a ModuleBuilder<'a, S, OL>,
        archiver: &'a A,
        internal_module_path_manager: &'a InternalModulePathManager<'a>,
    ) -> Self {
        Self {
            module_builder,
            archiver,
            internal_module_path_manager,
        }
    }

    pub fn build(&self, package: &ein::Package) -> Result<(), Box<dyn std::error::Error>> {
        let (object_file_path, interface_file_paths) = self.module_builder.build(package)?;

        self.archiver.archive(
            &object_file_path,
            self.internal_module_path_manager
                .archive_package_object_file_path(),
            &interface_file_paths
                .into_iter()
                .map(|interface_file_path| {
                    (
                        self.internal_module_path_manager
                            .convert_to_relative_interface_file_path(&interface_file_path),
                        interface_file_path,
                    )
                })
                .collect(),
        )?;

        Ok(())
    }
}
