use super::file_path::FilePath;

pub trait CommandLinker {
    fn link(
        &self,
        object_file_paths: &[FilePath],
        command_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
