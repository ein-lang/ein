use super::interface_linker::InterfaceLinker;
use super::path::FilePathManager;
use crate::infra::{FilePath, FileStorage, ObjectLinker};

pub struct PackageLinker<'a, S: FileStorage, OL: ObjectLinker> {
    object_linker: &'a OL,
    interface_linker: &'a InterfaceLinker<'a, S>,
    file_path_manager: &'a FilePathManager<'a>,
}

impl<'a, S: FileStorage, OL: ObjectLinker> PackageLinker<'a, S, OL> {
    pub fn new(
        object_linker: &'a OL,
        interface_linker: &'a InterfaceLinker<'a, S>,
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
    ) -> Result<(FilePath, FilePath), Box<dyn std::error::Error>> {
        let package_object_file_path = self
            .file_path_manager
            .configuration()
            .package_object_file_path();

        self.object_linker.link(
            object_file_paths
                .iter()
                .chain(external_package_object_file_paths),
            &package_object_file_path,
        )?;

        let package_interface_file_path = self
            .file_path_manager
            .configuration()
            .package_interface_file_path();

        self.interface_linker
            .link(interface_file_paths, &package_interface_file_path)?;

        Ok((
            package_object_file_path.clone(),
            package_interface_file_path.clone(),
        ))
    }
}
