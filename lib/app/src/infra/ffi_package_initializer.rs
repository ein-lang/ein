use crate::common::FilePath;

pub trait FfiPackageInitializer {
    fn initialize(
        &self,
        directory_path: &FilePath,
    ) -> Result<Option<FilePath>, Box<dyn std::error::Error>>;

    fn is_ffi_used(&self, directory_path: &FilePath) -> bool;
}
