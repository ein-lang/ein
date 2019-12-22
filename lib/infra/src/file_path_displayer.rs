use super::utilities;

#[derive(Default)]
pub struct FilePathDisplayer;

impl FilePathDisplayer {
    pub fn new() -> Self {
        Self
    }
}

impl app::FilePathDisplayer for FilePathDisplayer {
    fn display(&self, file_path: &app::FilePath) -> String {
        format!(
            "{}",
            utilities::convert_to_os_path(file_path)
                .canonicalize()
                .expect("valid os file path")
                .display()
        )
    }
}
