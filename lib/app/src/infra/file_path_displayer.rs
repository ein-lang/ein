use super::file_path::FilePath;

pub trait FilePathDisplayer {
    fn display(&self, file_path: &FilePath) -> String;
}

#[cfg(test)]
pub struct FakeFilePathDisplayer {}

#[cfg(test)]
impl FakeFilePathDisplayer {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
impl FilePathDisplayer for FakeFilePathDisplayer {
    fn display(&self, file_path: &FilePath) -> String {
        format!("{}", file_path)
    }
}
