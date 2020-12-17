use crate::path::FilePath;

pub trait ModuleObjectsLinker {
    fn link(
        &self,
        object_file_paths: &[FilePath],
        package_object_file_path: &FilePath,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
