use super::interface_linker::InterfaceLinker;
use super::path::FilePathManager;
use crate::infra::{FilePath, ObjectLinker};

pub struct PackageLinker<'a> {
    object_linker: &'a dyn ObjectLinker,
    interface_linker: &'a InterfaceLinker<'a>,
    file_path_manager: &'a FilePathManager<'a>,
}

impl<'a> PackageLinker<'a> {
    pub fn new(
        object_linker: &'a dyn ObjectLinker,
        interface_linker: &'a InterfaceLinker<'a>,
        file_path_manager: &'a FilePathManager<'a>,
    ) -> Self {
        Self {
            object_linker,
            interface_linker,
            file_path_manager,
        }
    }

    pub fn link(
        &self,
        object_file_paths: &[FilePath],
        external_package_object_file_paths: &[FilePath],
        interface_file_paths: &[FilePath],
    ) -> Result<FilePath, Box<dyn std::error::Error>> {
        let package_object_file_path = self
            .file_path_manager
            .configuration()
            .main_package_object_file_path();

        self.object_linker.link(
            &object_file_paths
                .iter()
                .chain(external_package_object_file_paths)
                .cloned()
                .collect::<Vec<_>>(),
            &package_object_file_path,
        )?;

        self.interface_linker.link(
            interface_file_paths,
            self.file_path_manager
                .configuration()
                .main_package_interface_file_path(),
        )?;

        Ok(package_object_file_path.clone())
    }
}
