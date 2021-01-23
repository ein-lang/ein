use crate::common::{FilePath, StaticFilePathManager};
use crate::infra::{FileSystem, ModuleObjectsLinker};

pub struct ModulesLinker<'a> {
    module_objects_linker: &'a dyn ModuleObjectsLinker,
    static_file_path_manager: &'a StaticFilePathManager,
    file_system: &'a dyn FileSystem,
}

impl<'a> ModulesLinker<'a> {
    pub fn new(
        module_objects_linker: &'a dyn ModuleObjectsLinker,
        static_file_path_manager: &'a StaticFilePathManager,
        file_system: &'a dyn FileSystem,
    ) -> Self {
        Self {
            module_objects_linker,
            static_file_path_manager,
            file_system,
        }
    }

    pub fn link(
        &self,
        object_file_paths: &[FilePath],
        interface_file_paths: &[FilePath],
        directory_path: &FilePath,
    ) -> Result<(FilePath, Vec<lang::ModuleInterface>), Box<dyn std::error::Error>> {
        let package_object_file_path =
            directory_path.join(self.static_file_path_manager.package_object_file_path());

        self.module_objects_linker
            .link(&object_file_paths, &package_object_file_path)?;

        Ok((
            package_object_file_path,
            interface_file_paths
                .into_iter()
                .map(|file_path| {
                    Ok(serde_json::from_slice(
                        &self.file_system.read_to_vec(&file_path)?,
                    )?)
                })
                .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?,
        ))
    }
}
