use crate::infra::FilePath;

const OBJECT_DIRECTORY: &str = "objects";
const EXTERNAL_PACKAGE_DIRECTORY: &str = "packages";

pub struct FilePathConfiguration {
    package_configuration_filename: String,
    package_object_filename: String,
    package_interface_filename: String,
    source_file_extension: String,
    object_file_extension: String,
    interface_file_extension: String,
    output_directory: FilePath,
    object_directory: FilePath,
    package_object_file_path: FilePath,
    archive_package_object_file_path: FilePath,
    package_interface_file_path: FilePath,
    archive_package_interface_file_path: FilePath,
    external_package_directory: FilePath,
}

impl FilePathConfiguration {
    pub fn new(
        package_configuration_filename: impl Into<String>,
        package_artifact_basename: impl Into<String> + std::fmt::Display,
        source_file_extension: impl Into<String> + std::fmt::Display,
        object_file_extension: impl Into<String> + std::fmt::Display,
        interface_file_extension: impl Into<String> + std::fmt::Display,
        output_directory: FilePath,
    ) -> Self {
        let package_object_filename =
            format!("{}.{}", package_artifact_basename, object_file_extension,);
        let package_interface_filename =
            format!("{}.{}", package_artifact_basename, interface_file_extension,);

        Self {
            package_configuration_filename: package_configuration_filename.into(),
            interface_file_extension: interface_file_extension.into(),
            package_object_file_path: output_directory
                .join(&FilePath::new(&[&package_object_filename])),
            archive_package_object_file_path: FilePath::new(&[&package_object_filename]),
            package_interface_file_path: output_directory
                .join(&FilePath::new(&[&package_interface_filename])),
            archive_package_interface_file_path: FilePath::new(&[&package_interface_filename]),
            external_package_directory: output_directory
                .join(&FilePath::new(&[EXTERNAL_PACKAGE_DIRECTORY])),
            object_directory: output_directory.join(&FilePath::new(&[OBJECT_DIRECTORY])),
            source_file_extension: source_file_extension.into(),
            object_file_extension: object_file_extension.into(),
            package_object_filename,
            package_interface_filename,
            output_directory,
        }
    }

    pub fn package_configuration_filename(&self) -> &str {
        &self.package_configuration_filename
    }

    pub fn package_object_filename(&self) -> &str {
        &self.package_object_filename
    }

    pub fn package_interface_filename(&self) -> &str {
        &self.package_interface_filename
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

    pub fn object_directory(&self) -> &FilePath {
        &self.object_directory
    }

    pub fn package_object_file_path(&self) -> &FilePath {
        &self.package_object_file_path
    }

    pub fn archive_package_object_file_path(&self) -> &FilePath {
        &self.archive_package_object_file_path
    }

    pub fn package_interface_file_path(&self) -> &FilePath {
        &self.package_interface_file_path
    }

    pub fn archive_package_interface_file_path(&self) -> &FilePath {
        &self.archive_package_interface_file_path
    }

    pub fn external_package_directory(&self) -> &FilePath {
        &self.external_package_directory
    }
}
