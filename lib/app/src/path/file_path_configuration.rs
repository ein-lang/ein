use crate::infra::FilePath;

const OBJECT_DIRECTORY: &str = "objects";
const EXTERNAL_PACKAGES_DIRECTORY: &str = "packages";
const PRELUDE_PACKAGE_DIRECTORY: &str = "prelude";

pub struct FilePathConfiguration {
    source_file_extension: String,
    object_file_extension: String,
    interface_file_extension: String,
    build_configuration_file_path: FilePath,
    output_directory_path: FilePath,
    object_directory_path: FilePath,
    package_object_file_path: FilePath,
    package_interface_file_path: FilePath,
    external_packages_directory_path: FilePath,
    prelude_package_directory_path: FilePath,
    main_source_file_path: FilePath,
}

impl FilePathConfiguration {
    pub fn new(
        build_configuration_filename: impl Into<String>,
        output_directory_name: impl Into<String>,
        package_artifact_basename: impl Into<String> + std::fmt::Display,
        source_file_extension: impl Into<String> + std::fmt::Display,
        object_file_extension: impl Into<String> + std::fmt::Display,
        interface_file_extension: impl Into<String> + std::fmt::Display,
        main_file_basename: impl Into<String>,
    ) -> Self {
        let output_directory_path = FilePath::new(&[output_directory_name.into()]);
        let package_object_filename =
            format!("{}.{}", package_artifact_basename, object_file_extension,);
        let package_interface_filename =
            format!("{}.{}", package_artifact_basename, interface_file_extension,);
        let external_packages_directory_path =
            output_directory_path.join(&FilePath::new(&[EXTERNAL_PACKAGES_DIRECTORY]));
        let source_file_extension = source_file_extension.into();

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
            main_source_file_path: FilePath::new(&[main_file_basename.into()])
                .with_extension(&source_file_extension),
            source_file_extension,
            object_file_extension: object_file_extension.into(),
            output_directory_path,
            build_configuration_file_path: FilePath::new(&[build_configuration_filename.into()]),
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

    pub fn output_directory_path(&self) -> &FilePath {
        &self.output_directory_path
    }

    pub fn object_directory_path(&self) -> &FilePath {
        &self.object_directory_path
    }

    pub fn package_object_file_path(&self) -> &FilePath {
        &self.package_object_file_path
    }

    pub fn package_interface_file_path(&self) -> &FilePath {
        &self.package_interface_file_path
    }

    pub fn external_packages_directory_path(&self) -> &FilePath {
        &self.external_packages_directory_path
    }

    pub fn prelude_package_directory_path(&self) -> &FilePath {
        &self.prelude_package_directory_path
    }

    pub fn main_source_file_path(&self) -> &FilePath {
        &self.main_source_file_path
    }
}
