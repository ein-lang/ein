use super::file_path::FilePath;

pub trait Archiver {
    fn archive(
        &self,
        object_file_paths: &[FilePath],
        interface_file_paths: &[FilePath],
    ) -> Result<(), std::io::Error>;
}
