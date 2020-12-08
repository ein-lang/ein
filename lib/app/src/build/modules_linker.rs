use super::module_interfaces_linker::ModuleInterfacesLinker;
use super::path::FilePathManager;
use crate::infra::{FilePath, ModuleObjectsLinker};

pub struct ModulesLinker<'a> {
    module_objects_linker: &'a dyn ModuleObjectsLinker,
    module_interfaces_linker: &'a ModuleInterfacesLinker<'a>,
    file_path_manager: &'a FilePathManager<'a>,
}

impl<'a> ModulesLinker<'a> {
    pub fn new(
        module_objects_linker: &'a dyn ModuleObjectsLinker,
        module_interfaces_linker: &'a ModuleInterfacesLinker<'a>,
        file_path_manager: &'a FilePathManager<'a>,
    ) -> Self {
        Self {
            module_objects_linker,
            module_interfaces_linker,
            file_path_manager,
        }
    }

    pub fn link(
        &self,
        object_file_paths: &[FilePath],
        interface_file_paths: &[FilePath],
        directory_path: &FilePath,
    ) -> Result<(FilePath, FilePath), Box<dyn std::error::Error>> {
        let package_object_file_path = directory_path.join(
            &self
                .file_path_manager
                .configuration()
                .package_object_file_path,
        );

        self.module_objects_linker
            .link(&object_file_paths, &package_object_file_path)?;

        let package_interface_file_path = directory_path.join(
            &self
                .file_path_manager
                .configuration()
                .package_interface_file_path,
        );

        self.module_interfaces_linker
            .link(interface_file_paths, &package_interface_file_path)?;

        Ok((package_object_file_path, package_interface_file_path))
    }
}
