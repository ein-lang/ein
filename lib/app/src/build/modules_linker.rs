use super::module_interfaces_linker::ModuleInterfacesLinker;
use super::package_interface::PackageInterface;
use crate::common::{FilePath, StaticFilePathManager};
use crate::infra::ModuleObjectsLinker;

pub struct ModulesLinker<'a> {
    module_objects_linker: &'a dyn ModuleObjectsLinker,
    module_interfaces_linker: &'a ModuleInterfacesLinker<'a>,
    static_file_path_manager: &'a StaticFilePathManager,
}

impl<'a> ModulesLinker<'a> {
    pub fn new(
        module_objects_linker: &'a dyn ModuleObjectsLinker,
        module_interfaces_linker: &'a ModuleInterfacesLinker<'a>,
        static_file_path_manager: &'a StaticFilePathManager,
    ) -> Self {
        Self {
            module_objects_linker,
            module_interfaces_linker,
            static_file_path_manager,
        }
    }

    pub fn link(
        &self,
        object_file_paths: &[FilePath],
        interface_file_paths: &[FilePath],
        directory_path: &FilePath,
    ) -> Result<(FilePath, PackageInterface), Box<dyn std::error::Error>> {
        let package_object_file_path =
            directory_path.join(self.static_file_path_manager.package_object_file_path());

        self.module_objects_linker
            .link(&object_file_paths, &package_object_file_path)?;

        Ok((
            package_object_file_path,
            self.module_interfaces_linker.link(interface_file_paths)?,
        ))
    }
}
