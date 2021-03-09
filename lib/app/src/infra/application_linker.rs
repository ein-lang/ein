use crate::common::FilePath;

pub trait ApplicationLinker {
    fn link(
        &self,
        object_file_paths: &[FilePath],
        application_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
