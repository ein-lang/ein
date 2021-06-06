use super::{
    file_path::FilePath,
    file_path_configuration::{
        FilePathConfiguration, EXTERNAL_PACKAGES_DIRECTORY, OBJECT_DIRECTORY,
        PRELUDE_PACKAGE_DIRECTORY,
    },
};

pub struct StaticFilePathManager {
    build_configuration_file_path: FilePath,
    object_directory_path: FilePath,
    external_packages_directory_path: FilePath,
    prelude_package_directory_path: FilePath,
    main_source_file_path: FilePath,
}

impl StaticFilePathManager {
    pub fn new(configuration: &FilePathConfiguration) -> Self {
        let output_directory_path = FilePath::new(&[configuration.output_directory_name]);
        let external_packages_directory_path =
            output_directory_path.join(&FilePath::new(&[EXTERNAL_PACKAGES_DIRECTORY]));

        Self {
            prelude_package_directory_path: external_packages_directory_path
                .join(&FilePath::new(&[PRELUDE_PACKAGE_DIRECTORY])),
            external_packages_directory_path,
            object_directory_path: output_directory_path.join(&FilePath::new(&[OBJECT_DIRECTORY])),
            main_source_file_path: FilePath::new(&[configuration.main_file_basename])
                .with_extension(configuration.source_file_extension),
            build_configuration_file_path: FilePath::new(&[
                configuration.build_configuration_filename
            ]),
        }
    }

    pub fn build_configuration_file_path(&self) -> &FilePath {
        &self.build_configuration_file_path
    }

    pub fn object_directory_path(&self) -> &FilePath {
        &self.object_directory_path
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
