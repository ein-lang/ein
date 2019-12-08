use super::file_path::FilePath;

pub trait ObjectLinker {
    fn link<'a>(
        &self,
        object_file_paths: impl IntoIterator<Item = &'a FilePath>,
        package_object_file_path: &FilePath,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
