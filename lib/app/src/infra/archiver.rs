use super::file_path::FilePath;
use std::collections::HashMap;

pub trait Archiver {
    fn archive(
        &self,
        object_file_paths: &[FilePath],
        interface_file_paths: &HashMap<FilePath, FilePath>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
