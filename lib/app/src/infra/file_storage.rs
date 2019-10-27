use super::file_path::FilePath;

pub trait FileStorage {
    fn write(&self, path: &FilePath, data: &[u8]) -> Result<(), std::io::Error>;
    fn read_to_vec(&self, path: &FilePath) -> Result<Vec<u8>, std::io::Error>;
    fn read_to_string(&self, path: &FilePath) -> Result<String, std::io::Error>;
    fn exists(&self, path: &FilePath) -> bool;
}
