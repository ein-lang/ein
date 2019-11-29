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
        FilePath::new(internal_module_path.components().to_vec())
            .with_extension(self.file_path_configuration.source_file_extension())
    }

    pub fn resolve_to_interface_file_path(
        &self,
        internal_module_path: &ein::InternalUnresolvedModulePath,
    ) -> FilePath {
        FilePath::new(
            vec![
                self.file_path_configuration.output_directory().into(),
                INTERFACE_DIRECTORY.into(),
            ]
            .drain(..)
            .chain(internal_module_path.components().iter().cloned())
            .collect(),
        )
        .with_extension(self.file_path_configuration.interface_file_extension())
    }

    pub fn convert_to_object_file_path(&self, module_path: &ein::ModulePath) -> FilePath {
        FilePath::new(
            vec![
                self.file_path_configuration.output_directory().into(),
                OBJECT_DIRECTORY.into(),
            ]
            .drain(..)
            .chain(module_path.components().iter().cloned())
            .collect(),
        )
        .with_extension(self.file_path_configuration.object_file_extension())
    }

    pub fn convert_to_interface_file_path(&self, module_path: &ein::ModulePath) -> FilePath {
        FilePath::new(
            vec![
                self.file_path_configuration.output_directory().into(),
                INTERFACE_DIRECTORY.into(),
            ]
            .drain(..)
            .chain(module_path.components().iter().cloned())
            .collect(),
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
            source_file_path.with_extension("").components().to_vec(),
        )
    }

    pub fn convert_to_relative_interface_file_path(&self, source_file_path: &FilePath) -> FilePath {
        FilePath::new(
            source_file_path
                .components()
                .iter()
                .skip(2)
                .cloned()
                .collect(),
        )
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
            FilePath::new(vec![
                "target".into(),
                "interfaces".into(),
                "package".into(),
                "Foo.interface".into()
            ])
        );
    }

    #[test]
    fn convert_to_relative_interface_file_path() {
        assert_eq!(
            InternalModulePathManager::new(&FilePathConfiguration::new("target", "", "", ""))
                .convert_to_relative_interface_file_path(&FilePath::new(vec![
                    "target".into(),
                    "interfaces".into(),
                    "package".into(),
                    "Foo.interface".into()
                ])),
            FilePath::new(vec!["package".into(), "Foo.interface".into()])
        );
    }
}
