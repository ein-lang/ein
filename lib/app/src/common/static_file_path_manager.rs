use super::file_path::FilePath;
use super::file_path_configuration::{
    FilePathConfiguration, EXTERNAL_PACKAGES_DIRECTORY, OBJECT_DIRECTORY, PRELUDE_PACKAGE_DIRECTORY,
};

pub struct StaticFilePathManager {
    build_configuration_file_path: FilePath,
    output_directory_path: FilePath,
    object_directory_path: FilePath,
    package_object_file_path: FilePath,
    package_interface_file_path: FilePath,
    external_packages_directory_path: FilePath,
    prelude_package_directory_path: FilePath,
    main_source_file_path: FilePath,
}

impl StaticFilePathManager {
    pub fn new(configuration: &FilePathConfiguration) -> Self {
        let output_directory_path = FilePath::new(&[configuration.output_directory_name.clone()]);
        let package_object_filename = format!(
            "{}.{}",
            &configuration.package_artifact_basename, &configuration.object_file_extension,
        );
        let package_interface_filename = format!(
            "{}.{}",
            &configuration.package_artifact_basename, &configuration.interface_file_extension,
        );
        let external_packages_directory_path =
            output_directory_path.join(&FilePath::new(&[EXTERNAL_PACKAGES_DIRECTORY]));

        Self {
            package_object_file_path: output_directory_path
                .join(&FilePath::new(&[&package_object_filename])),
            package_interface_file_path: output_directory_path
                .join(&FilePath::new(&[&package_interface_filename])),
            prelude_package_directory_path: external_packages_directory_path
                .join(&FilePath::new(&[PRELUDE_PACKAGE_DIRECTORY])),
            external_packages_directory_path,
            object_directory_path: output_directory_path.join(&FilePath::new(&[OBJECT_DIRECTORY])),
            main_source_file_path: FilePath::new(&[configuration.main_file_basename.clone()])
                .with_extension(&configuration.source_file_extension),
            output_directory_path,
            build_configuration_file_path: FilePath::new(&[configuration
                .build_configuration_filename
                .clone()]),
        }
    }

    pub fn build_configuration_file_path(&self) -> &FilePath {
        &self.build_configuration_file_path
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
