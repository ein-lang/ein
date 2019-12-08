use super::file_path::FilePath;

pub trait LibraryArchiver {
    fn archive(
        &self,
        object_file_path: &FilePath,
        archive_object_file_path: &FilePath,
        interface_file_path: &FilePath,
        archive_interface_file_path: &FilePath,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
