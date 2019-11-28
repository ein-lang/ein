use super::file_path::FilePath;

pub trait ExternalPackageDownloader {
    fn download(
        &self,
        name: &str,
        version: &str,
        directory_path: &FilePath,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
