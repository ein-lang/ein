use super::file_path::FilePath;

pub trait ExternalPackageBuilder {
    fn build(&self, directory_path: &FilePath) -> Result<(), Box<dyn std::error::Error>>;
}

#[cfg(test)]
pub struct FakeExternalPackageBuilder<'a, S: crate::infra::FileStorage> {
    file_path_configuration: &'a crate::FilePathConfiguration,
    file_storage: &'a S,
}

#[cfg(test)]
impl<'a, S: crate::infra::FileStorage> FakeExternalPackageBuilder<'a, S> {
    pub fn new(
        file_path_configuration: &'a crate::FilePathConfiguration,
        file_storage: &'a S,
    ) -> Self {
        Self {
            file_path_configuration,
            file_storage,
        }
    }
}

#[cfg(test)]
impl<'a, S: crate::infra::FileStorage> ExternalPackageBuilder
    for FakeExternalPackageBuilder<'a, S>
{
    fn build(&self, directory_path: &FilePath) -> Result<(), Box<dyn std::error::Error>> {
        self.file_storage.write(
            &directory_path.join(self.file_path_configuration.main_package_object_file_path()),
            &[],
        )?;
        self.file_storage.write(
            &directory_path.join(
                self.file_path_configuration
                    .main_package_interface_file_path(),
            ),
            serde_json::to_string(&crate::PackageInterface::new(&[]))?.as_bytes(),
        )?;

        Ok(())
    }
}
