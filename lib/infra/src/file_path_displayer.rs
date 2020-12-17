use super::os_file_path_converter::OsFilePathConverter;

pub struct FilePathDisplayer<'a> {
    os_file_path_converter: &'a OsFilePathConverter,
}

impl<'a> FilePathDisplayer<'a> {
    pub fn new(os_file_path_converter: &'a OsFilePathConverter) -> Self {
        Self {
            os_file_path_converter,
        }
    }
}

impl<'a> app::FilePathDisplayer for FilePathDisplayer<'a> {
    fn display(&self, file_path: &app::FilePath) -> String {
        format!(
            "{}",
            self.os_file_path_converter
                .convert_to_os_path(file_path)
                .canonicalize()
                .expect("valid os file path")
                .display()
        )
    }
}
