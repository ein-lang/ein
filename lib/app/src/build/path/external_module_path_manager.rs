use crate::build::FilePathConfiguration;
use crate::infra::FilePath;

const EXTERNAL_PACKAGE_DIRECTORY: &str = "packages";

pub struct ExternalModulePathManager {
    external_package_directory: FilePath,
}

impl ExternalModulePathManager {
    pub fn new(file_path_configuration: &FilePathConfiguration) -> Self {
        ExternalModulePathManager {
            external_package_directory: file_path_configuration
                .output_directory()
                .join(&FilePath::new(&[EXTERNAL_PACKAGE_DIRECTORY])),
        }
    }

    pub fn convert_to_directory_path(&self, package_name: &str) -> FilePath {
        self.external_package_directory
            .join(&FilePath::new(package_name.split('/')))
    }
}
