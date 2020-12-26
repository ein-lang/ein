use crate::common::FilePath;

pub trait FfiPackageInitializer {
    fn initialize(&self, directory_path: &FilePath) -> Result<(), Box<dyn std::error::Error>>;
}
