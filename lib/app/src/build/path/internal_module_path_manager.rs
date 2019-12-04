use crate::build::FilePathConfiguration;
use crate::infra::FilePath;

const OBJECT_DIRECTORY: &str = "objects";
const INTERFACE_DIRECTORY: &str = "interfaces";

pub struct InternalModulePathManager<'a> {
    file_path_configuration: &'a FilePathConfiguration,
    object_directory: FilePath,
    interface_directory: FilePath,
    source_file_glob_pattern: String,
}

impl<'a> InternalModulePathManager<'a> {
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

    pub fn convert_to_relative_interface_file_path(&self, source_file_path: &FilePath) -> FilePath {
        FilePath::new(source_file_path.components().skip(2))
    }

    pub fn source_file_glob_pattern(&self) -> &str {
        &self.source_file_glob_pattern
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_to_interface_file_path() {
        assert_eq!(
            InternalModulePathManager::new(&FilePathConfiguration::new(
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

    #[test]
    fn convert_to_relative_interface_file_path() {
        assert_eq!(
            InternalModulePathManager::new(&FilePathConfiguration::new(
                "",
                "",
                "",
                "",
                FilePath::new(&["target"])
            ))
            .convert_to_relative_interface_file_path(&FilePath::new(&[
                "target",
                "interfaces",
                "package",
                "Foo.interface"
            ])),
            FilePath::new(&["package", "Foo.interface"])
        );
    }
}
