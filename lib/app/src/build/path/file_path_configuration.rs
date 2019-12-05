use crate::infra::FilePath;

pub struct FilePathConfiguration {
    package_configuration_filename: String,
    package_object_filename: String,
    source_file_extension: String,
    object_file_extension: String,
    interface_file_extension: String,
    output_directory: FilePath,
}

impl FilePathConfiguration {
    pub fn new(
        package_configuration_filename: impl Into<String>,
        package_object_basename: impl Into<String> + std::fmt::Display,
        source_file_extension: impl Into<String>,
        object_file_extension: impl Into<String> + std::fmt::Display,
        interface_file_extension: impl Into<String>,
        output_directory: FilePath,
    ) -> Self {
        Self {
            package_configuration_filename: package_configuration_filename.into(),
            package_object_filename: format!(
                "{}.{}",
                package_object_basename, object_file_extension,
            ),
            source_file_extension: source_file_extension.into(),
            object_file_extension: object_file_extension.into(),
            interface_file_extension: interface_file_extension.into(),
            output_directory,
        }
    }

    pub fn package_configuration_filename(&self) -> &str {
        &self.package_configuration_filename
    }

    pub fn package_object_filename(&self) -> &str {
        &self.package_object_filename
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

    pub fn output_directory(&self) -> &FilePath {
        &self.output_directory
    }
}
