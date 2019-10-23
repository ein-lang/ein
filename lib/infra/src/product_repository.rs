use super::error::RepositoryError;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub struct ProductRepository {
    product_directory: Box<Path>,
}

impl ProductRepository {
    pub fn new(product_directory: &str) -> Self {
        Self {
            product_directory: Path::new(product_directory).into(),
        }
    }

    pub fn load(&self, path: impl AsRef<Path>, vec: &mut Vec<u8>) -> Result<(), RepositoryError> {
        File::open(self.product_directory.join(path))?.read_to_end(vec)?;

        Ok(())
    }

    pub fn store(&self, path: impl AsRef<Path>, data: &[u8]) -> Result<(), RepositoryError> {
        File::create(self.product_directory.join(path))?.write_all(data)?;

        Ok(())
    }
}
