use super::file_path::FilePath;
use crate::build::ExternalPackageId;

pub trait ExternalPackageDownloader {
    fn download(
        &self,
        external_package_id: &ExternalPackageId,
        directory_path: &FilePath,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

#[cfg(test)]
pub struct FakeExternalPackageDownloader<'a, S: crate::infra::FileStorage> {
    packages: std::collections::HashMap<String, std::collections::HashMap<FilePath, Vec<u8>>>,
    file_storage: &'a S,
}

#[cfg(test)]
impl<'a, S: crate::infra::FileStorage> FakeExternalPackageDownloader<'a, S> {
    pub fn new(
        packages: std::collections::HashMap<String, std::collections::HashMap<FilePath, Vec<u8>>>,
        file_storage: &'a S,
    ) -> Self {
        Self {
            packages,
            file_storage,
        }
    }
}

#[cfg(test)]
impl<'a, S: crate::infra::FileStorage> ExternalPackageDownloader
    for FakeExternalPackageDownloader<'a, S>
{
    fn download(
        &self,
        external_package_id: &ExternalPackageId,
        directory_path: &FilePath,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (path, data) in &self.packages[external_package_id.name()] {
            self.file_storage.write(&directory_path.join(&path), data)?;
        }

        Ok(())
    }
}
