use crate::infra::FilePath;

const OBJECT_DIRECTORY: &str = "objects";
const EXTERNAL_PACKAGES_DIRECTORY: &str = "packages";
const PRELUDE_PACKAGE_DIRECTORY: &str = "prelude";

pub struct FilePathConfiguration {
    pub source_file_extension: String,
    pub object_file_extension: String,
    pub interface_file_extension: String,
    pub build_configuration_file_path: FilePath,
    pub output_directory_path: FilePath,
    pub object_directory_path: FilePath,
    pub package_object_file_path: FilePath,
    pub package_interface_file_path: FilePath,
    pub external_packages_directory_path: FilePath,
    pub prelude_package_directory_path: FilePath,
}

impl FilePathConfiguration {
    pub fn new(
        build_configuration_filename: impl Into<String>,
        output_directory_name: impl Into<String>,
        package_artifact_basename: impl Into<String> + std::fmt::Display,
        source_file_extension: impl Into<String> + std::fmt::Display,
        object_file_extension: impl Into<String> + std::fmt::Display,
        interface_file_extension: impl Into<String> + std::fmt::Display,
    ) -> Self {
        let output_directory_path = FilePath::new(&[output_directory_name.into()]);
        let package_object_filename =
            format!("{}.{}", package_artifact_basename, object_file_extension,);
        let package_interface_filename =
            format!("{}.{}", package_artifact_basename, interface_file_extension,);
        let external_packages_directory_path =
            output_directory_path.join(&FilePath::new(&[EXTERNAL_PACKAGES_DIRECTORY]));

        Self {
            interface_file_extension: interface_file_extension.into(),
            package_object_file_path: output_directory_path
                .join(&FilePath::new(&[&package_object_filename])),
            package_interface_file_path: output_directory_path
                .join(&FilePath::new(&[&package_interface_filename])),
            prelude_package_directory_path: external_packages_directory_path
                .join(&FilePath::new(&[PRELUDE_PACKAGE_DIRECTORY])),
            external_packages_directory_path,
            object_directory_path: output_directory_path.join(&FilePath::new(&[OBJECT_DIRECTORY])),
            source_file_extension: source_file_extension.into(),
            object_file_extension: object_file_extension.into(),
            output_directory_path,
            build_configuration_file_path: FilePath::new(&[build_configuration_filename.into()]),
        }
    }
}
