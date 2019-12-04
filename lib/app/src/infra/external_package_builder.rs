use super::file_path::FilePath;

pub trait ExternalPackageBuilder {
    fn build(&self, directory_path: &FilePath) -> Result<(), Box<dyn std::error::Error>>;
}

#[cfg(test)]
pub struct ExternalPackageBuilderStub;

#[cfg(test)]
impl ExternalPackageBuilderStub {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(test)]
impl ExternalPackageBuilder for ExternalPackageBuilderStub {
    fn build(&self, _: &FilePath) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
