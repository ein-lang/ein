use super::file_path::FilePath;

pub trait ExternalPackageBuilder {
    fn build(&self, directory_path: &FilePath) -> Result<(), Box<dyn std::error::Error>>;
}

#[cfg(test)]
pub struct ExternalPackageBuilderFake<'a, S: crate::infra::FileStorage> {
    file_path_configuration: &'a crate::FilePathConfiguration,
    file_storage: &'a S,
}

#[cfg(test)]
impl<'a, S: crate::infra::FileStorage> ExternalPackageBuilderFake<'a, S> {
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
    for ExternalPackageBuilderFake<'a, S>
{
    fn build(&self, directory_path: &FilePath) -> Result<(), Box<dyn std::error::Error>> {
        self.file_storage.write(
            &directory_path.join(&FilePath::new(&[self
                .file_path_configuration
                .package_object_filename()])),
            &[],
        )?;
        self.file_storage.write(
            &directory_path.join(&FilePath::new(&[self
                .file_path_configuration
                .package_interface_filename()])),
            serde_json::to_string(&crate::PackageInterface::new(&[]))?.as_bytes(),
        )?;

        Ok(())
    }
}
