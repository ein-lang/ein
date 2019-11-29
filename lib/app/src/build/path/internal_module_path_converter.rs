use crate::build::FilePathConfiguration;
use crate::infra::FilePath;

const OBJECT_DIRECTORY: &str = "objects";
const INTERFACE_DIRECTORY: &str = "interfaces";

pub struct InternalModulePathManager<'a> {
    file_path_configuration: &'a FilePathConfiguration,
    source_file_glob_pattern: String,
}

impl<'a> InternalModulePathManager<'a> {
    pub fn new(file_path_configuration: &'a FilePathConfiguration) -> Self {
        Self {
            file_path_configuration,
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
        FilePath::new(
            vec![
                self.file_path_configuration.output_directory(),
                INTERFACE_DIRECTORY,
            ]
            .into_iter()
            .chain(internal_module_path.components()),
        )
        .with_extension(self.file_path_configuration.interface_file_extension())
    }

    pub fn convert_to_object_file_path(&self, module_path: &ein::ModulePath) -> FilePath {
        FilePath::new(
            vec![
                self.file_path_configuration.output_directory(),
                OBJECT_DIRECTORY,
            ]
            .into_iter()
            .chain(module_path.components()),
        )
        .with_extension(self.file_path_configuration.object_file_extension())
    }

    pub fn convert_to_interface_file_path(&self, module_path: &ein::ModulePath) -> FilePath {
        FilePath::new(
            vec![
                self.file_path_configuration.output_directory(),
                INTERFACE_DIRECTORY,
            ]
            .drain(..)
            .chain(module_path.components()),
        )
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
                "target",
                "",
                "",
                "interface"
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
            InternalModulePathManager::new(&FilePathConfiguration::new("target", "", "", ""))
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
