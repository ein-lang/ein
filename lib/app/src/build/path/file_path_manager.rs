use crate::build::FilePathConfiguration;
use crate::infra::FilePath;

const OBJECT_DIRECTORY: &str = "objects";
const INTERFACE_DIRECTORY: &str = "interfaces";
const EXTERNAL_PACKAGE_DIRECTORY: &str = "packages";

pub struct FilePathManager<'a> {
    file_path_configuration: &'a FilePathConfiguration,
    object_directory: FilePath,
    interface_directory: FilePath,
    source_file_glob_pattern: String,
    package_object_file_path: FilePath,
    archive_package_object_file_path: FilePath,
    package_interface_file_path: FilePath,
    archive_package_interface_file_path: FilePath,
    external_package_directory: FilePath,
}

impl<'a> FilePathManager<'a> {
    pub fn new(file_path_configuration: &'a FilePathConfiguration) -> Self {
        Self {
            file_path_configuration,
            object_directory: file_path_configuration
                .output_directory()
                .join(&FilePath::new(&[OBJECT_DIRECTORY])),
            interface_directory: file_path_configuration
                .output_directory()
                .join(&FilePath::new(&[INTERFACE_DIRECTORY])),
            source_file_glob_pattern: format!(
                "**/*.{}",
                file_path_configuration.source_file_extension()
            ),
            package_object_file_path: file_path_configuration.output_directory().join(
                &FilePath::new(&[file_path_configuration.package_object_filename()]),
            ),
            archive_package_object_file_path: FilePath::new(&[
                file_path_configuration.package_object_filename()
            ]),
            package_interface_file_path: file_path_configuration.output_directory().join(
                &FilePath::new(&[file_path_configuration.package_interface_filename()]),
            ),
            archive_package_interface_file_path: FilePath::new(&[
                file_path_configuration.package_interface_filename()
            ]),
            external_package_directory: file_path_configuration
                .output_directory()
                .join(&FilePath::new(&[EXTERNAL_PACKAGE_DIRECTORY])),
        }
    }

    pub fn resolve_to_source_file_path(
        &self,
        internal_module_path: &ein::InternalUnresolvedModulePath,
    ) -> FilePath {
        FilePath::new(internal_module_path.components())
            .with_extension(self.file_path_configuration.source_file_extension())
    }

    pub fn resolve_to_interface_file_path(
        &self,
        internal_module_path: &ein::InternalUnresolvedModulePath,
    ) -> FilePath {
        self.interface_directory
            .join(&FilePath::new(internal_module_path.components()))
            .with_extension(self.file_path_configuration.interface_file_extension())
    }

    pub fn convert_to_object_file_path(&self, module_path: &ein::ModulePath) -> FilePath {
        self.object_directory
            .join(&FilePath::new(module_path.components()))
            .with_extension(self.file_path_configuration.object_file_extension())
    }

    pub fn convert_to_interface_file_path(&self, module_path: &ein::ModulePath) -> FilePath {
        self.interface_directory
            .join(&FilePath::new(module_path.components()))
            .with_extension(self.file_path_configuration.interface_file_extension())
    }

    pub fn convert_to_module_path(
        &self,
        source_file_path: &FilePath,
        package: &ein::Package,
    ) -> ein::ModulePath {
        ein::ModulePath::new(
            package.clone(),
            source_file_path
                .with_extension("")
                .components()
                .map(String::from)
                .collect(),
        )
    }

    pub fn source_file_glob_pattern(&self) -> &str {
        &self.source_file_glob_pattern
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

    pub fn convert_to_directory_path(&self, package_name: &str) -> FilePath {
        self.external_package_directory
            .join(&FilePath::new(package_name.split('/')))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_to_interface_file_path() {
        assert_eq!(
            FilePathManager::new(&FilePathConfiguration::new(
                "",
                "",
                "",
                "",
                "interface",
                FilePath::new(&["target"])
            ))
            .resolve_to_interface_file_path(&ein::InternalUnresolvedModulePath::new(
                vec!["package".into(), "Foo".into()]
            )),
            FilePath::new(&["target", "interfaces", "package", "Foo.interface"])
        );
    }
}
