use super::file_path::FilePath;

pub trait Linker {
    fn link(&self, object_file_path: &FilePath) -> Result<(), std::io::Error>;
}
