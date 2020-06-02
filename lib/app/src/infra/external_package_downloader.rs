use super::file_path::FilePath;

pub trait ExternalPackageDownloader {
    fn download(
        &self,
        name: &str,
        version: &str,
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
        name: &str,
        _: &str,
        directory_path: &FilePath,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (path, data) in &self.packages[name] {
            self.file_storage.write(&directory_path.join(&path), data)?;
        }

        Ok(())
    }
}
