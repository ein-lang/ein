use super::module_builder::ModuleBuilder;
use super::path::InternalModulePathConverter;
use crate::infra::{Archiver, FileStorage};

pub struct LibraryPackageBuilder<'a, S: FileStorage, A: Archiver> {
    module_builder: &'a ModuleBuilder<'a, S>,
    archiver: &'a A,
    internal_module_path_converter: &'a InternalModulePathConverter<'a>,
}

impl<'a, S: FileStorage, A: Archiver> LibraryPackageBuilder<'a, S, A> {
    pub fn new(
        module_builder: &'a ModuleBuilder<'a, S>,
        archiver: &'a A,
        internal_module_path_converter: &'a InternalModulePathConverter<'a>,
    ) -> Self {
        Self {
            module_builder,
            archiver,
            internal_module_path_converter,
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
                        self.internal_module_path_converter
                            .convert_to_relative_interface_file_path(&interface_file_path),
                        interface_file_path,
                    )
                })
                .collect(),
        )?;

        Ok(())
    }
}
