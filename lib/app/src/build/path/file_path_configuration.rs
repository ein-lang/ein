use crate::infra::FilePath;

pub struct FilePathConfiguration {
    output_directory: FilePath,
    source_file_extension: String,
    object_file_extension: String,
    interface_file_extension: String,
}

impl FilePathConfiguration {
    pub fn new(
        output_directory: FilePath,
        source_file_extension: impl Into<String>,
        object_file_extension: impl Into<String>,
        interface_file_extension: impl Into<String>,
    ) -> Self {
        Self {
            output_directory,
            source_file_extension: source_file_extension.into(),
            object_file_extension: object_file_extension.into(),
            interface_file_extension: interface_file_extension.into(),
        }
    }

    pub fn output_directory(&self) -> &FilePath {
        &self.output_directory
    }

    pub fn source_file_extension(&self) -> &str {
        &self.source_file_extension
    }

    pub fn object_file_extension(&self) -> &str {
        &self.object_file_extension
    }

    pub fn interface_file_extension(&self) -> &str {
        &self.interface_file_extension
    }
}
