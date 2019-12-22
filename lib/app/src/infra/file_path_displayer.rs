use super::file_path::FilePath;

pub trait FilePathDispalyer {
    fn display(&self, file_path: &FilePath) -> String;
}

#[cfg(test)]
pub struct FilePathDispalyerFake {}

#[cfg(test)]
impl FilePathDispalyerFake {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
impl FilePathDispalyer for FilePathDispalyerFake {
    fn display(&self, file_path: &FilePath) -> String {
        format!("{}", file_path)
    }
}
