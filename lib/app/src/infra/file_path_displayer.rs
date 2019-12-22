use super::file_path::FilePath;

pub trait FilePathDisplayer {
    fn display(&self, file_path: &FilePath) -> String;
}

#[cfg(test)]
pub struct FilePathDisplayerFake {}

#[cfg(test)]
impl FilePathDisplayerFake {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
impl FilePathDisplayer for FilePathDisplayerFake {
    fn display(&self, file_path: &FilePath) -> String {
        format!("{}", file_path)
    }
}
