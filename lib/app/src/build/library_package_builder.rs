use super::module_builder::ModuleBuilder;
use super::path::InternalModulePathManager;
use crate::infra::{FileStorage, LibraryArchiver};

pub struct LibraryPackageBuilder<'a, S: FileStorage, A: LibraryArchiver> {
    module_builder: &'a ModuleBuilder<'a, S>,
    archiver: &'a A,
    internal_module_path_manager: &'a InternalModulePathManager<'a>,
}

impl<'a, S: FileStorage, A: LibraryArchiver> LibraryPackageBuilder<'a, S, A> {
    pub fn new(
        module_builder: &'a ModuleBuilder<'a, S>,
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
        let mut file_paths = self.module_builder.build(package)?;

        self.archiver.archive(
            &file_paths
                .iter()
                .map(|(object_file_path, _)| object_file_path)
                .cloned()
                .collect::<Vec<_>>(),
            &file_paths
                .drain(..)
                .map(|(_, interface_file_path)| {
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
