use super::file_path::FilePath;
use std::collections::HashMap;

pub trait LibraryArchiver {
    fn archive(
        &self,
        object_file_path: &FilePath,
        interface_file_paths: &HashMap<FilePath, FilePath>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
