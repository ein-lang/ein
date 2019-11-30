use crate::build::FilePathConfiguration;
use crate::infra::FilePath;

const EXTERNAL_PACKAGE_DIRECTORY: &str = "packages";

pub struct ExternalModulePathManager<'a> {
    file_path_configuration: &'a FilePathConfiguration,
    external_package_directory: FilePath,
}

impl<'a> ExternalModulePathManager<'a> {
    pub fn new(file_path_configuration: &'a FilePathConfiguration) -> Self {
        ExternalModulePathManager {
            file_path_configuration,
            external_package_directory: file_path_configuration
                .output_directory()
                .join(FilePath::new(&[EXTERNAL_PACKAGE_DIRECTORY])),
        }
    }

    pub fn resolve_to_interface_file_path(
        &self,
        external_module_path: &ein::ExternalUnresolvedModulePath,
    ) -> FilePath {
        self.external_package_directory
            .join(FilePath::new(external_module_path.components()))
            .with_extension(self.file_path_configuration.interface_file_extension())
    }

    pub fn convert_to_directory_path(&self, package_name: &str) -> FilePath {
        self.external_package_directory
            .join(FilePath::new(package_name.split('/')))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_from_file_path() {
        assert_eq!(
            ExternalModulePathManager::new(&FilePathConfiguration::new(
                FilePath::new(&["target"]),
                "",
                "",
                "json"
            ))
            .resolve_to_interface_file_path(&ein::ExternalUnresolvedModulePath::new(
                vec!["package".into(), "Foo".into()]
            )),
            FilePath::new(&["target", "packages", "package", "Foo.json"])
        );
    }
}
