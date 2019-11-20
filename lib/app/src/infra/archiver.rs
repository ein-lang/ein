use super::file_path::FilePath;

pub trait Archiver {
    fn archive(
        &self,
        object_file_paths: &[FilePath],
        interface_file_paths: &[FilePath],
        library_name: &str,
    ) -> Result<(), std::io::Error>;
}
