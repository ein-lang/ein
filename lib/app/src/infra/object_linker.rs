use super::file_path::FilePath;

pub trait ObjectLinker {
    fn link(
        &self,
        object_file_paths: &[FilePath],
        package_object_file_path: &FilePath,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
