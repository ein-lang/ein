use crate::common::FilePath;

pub trait PreludePackageDownloader {
    fn download(
        &self,
        external_packages_directory_path: &FilePath,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
