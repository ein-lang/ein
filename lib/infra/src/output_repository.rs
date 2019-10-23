use super::repository_error::RepositoryError;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub struct OutputRepository {
    output_directory: Box<Path>,
}

impl OutputRepository {
    pub fn new(output_directory: &str) -> Self {
        Self {
            output_directory: Path::new(output_directory).into(),
        }
    }

    pub fn load(&self, path: impl AsRef<Path>, vec: &mut Vec<u8>) -> Result<(), RepositoryError> {
        File::open(self.output_directory.join(path))?.read_to_end(vec)?;

        Ok(())
    }

    pub fn store(&self, path: impl AsRef<Path>, data: &[u8]) -> Result<(), RepositoryError> {
        File::create(self.output_directory.join(path))?.write_all(data)?;

        Ok(())
    }
}
