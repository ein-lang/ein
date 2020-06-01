use crate::infra::FilePath;

const OBJECT_DIRECTORY: &str = "objects";

pub struct FilePathConfiguration {
    source_file_extension: String,
    object_file_extension: String,
    interface_file_extension: String,
    build_configuration_file_path: FilePath,
    output_directory: FilePath,
    object_directory: FilePath,
    main_package_object_file_path: FilePath,
    main_package_interface_file_path: FilePath,
    package_object_file_path: FilePath,
    package_interface_file_path: FilePath,
    external_package_directory: FilePath,
}

impl FilePathConfiguration {
    pub fn new(
        build_configuration_file_path: impl Into<String>,
        package_artifact_basename: impl Into<String> + std::fmt::Display,
        source_file_extension: impl Into<String> + std::fmt::Display,
        object_file_extension: impl Into<String> + std::fmt::Display,
        interface_file_extension: impl Into<String> + std::fmt::Display,
        output_directory: FilePath,
        external_package_directory: FilePath,
    ) -> Self {
        let package_object_filename =
            format!("{}.{}", package_artifact_basename, object_file_extension,);
        let package_interface_filename =
            format!("{}.{}", package_artifact_basename, interface_file_extension,);

        Self {
            interface_file_extension: interface_file_extension.into(),
            main_package_object_file_path: output_directory
                .join(&FilePath::new(&[&package_object_filename])),
            main_package_interface_file_path: output_directory
                .join(&FilePath::new(&[&package_interface_filename])),
            package_object_file_path: FilePath::new(&[&package_object_filename]),
            package_interface_file_path: FilePath::new(&[&package_interface_filename]),
            external_package_directory,
            object_directory: output_directory.join(&FilePath::new(&[OBJECT_DIRECTORY])),
            source_file_extension: source_file_extension.into(),
            object_file_extension: object_file_extension.into(),
            output_directory,
            build_configuration_file_path: FilePath::new(&[build_configuration_file_path.into()]),
        }
    }

    pub fn build_configuration_file_path(&self) -> &FilePath {
        &self.build_configuration_file_path
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

    pub fn main_package_object_file_path(&self) -> &FilePath {
        &self.main_package_object_file_path
    }

    pub fn main_package_interface_file_path(&self) -> &FilePath {
        &self.main_package_interface_file_path
    }

    pub fn package_object_file_path(&self) -> &FilePath {
        &self.package_object_file_path
    }

    pub fn package_interface_file_path(&self) -> &FilePath {
        &self.package_interface_file_path
    }

    pub fn external_package_directory(&self) -> &FilePath {
        &self.external_package_directory
    }
}
