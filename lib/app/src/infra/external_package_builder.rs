use super::file_path::FilePath;

pub trait ExternalPackageBuilder {
    fn build(&self, directory_path: &FilePath) -> Result<(), Box<dyn std::error::Error>>;
}
