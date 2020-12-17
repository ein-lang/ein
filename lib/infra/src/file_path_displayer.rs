use super::file_path_converter::FilePathConverter;

pub struct FilePathDisplayer<'a> {
    file_path_converter: &'a FilePathConverter,
}

impl<'a> FilePathDisplayer<'a> {
    pub fn new(file_path_converter: &'a FilePathConverter) -> Self {
        Self {
            file_path_converter,
        }
    }
}

impl<'a> app::FilePathDisplayer for FilePathDisplayer<'a> {
    fn display(&self, file_path: &app::FilePath) -> String {
        format!(
            "{}",
            self.file_path_converter
                .convert_to_os_path(file_path)
                .canonicalize()
                .expect("valid os file path")
                .display()
        )
    }
}
