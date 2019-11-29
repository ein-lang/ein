use crate::build::FilePathConfiguration;
use crate::infra::FilePath;

const EXTERNAL_PACKAGE_DIRECTORY: &str = "packages";

pub struct ExternalModulePathManager<'a> {
    file_path_configuration: &'a FilePathConfiguration,
}

impl<'a> ExternalModulePathManager<'a> {
    pub fn new(file_path_configuration: &'a FilePathConfiguration) -> Self {
        ExternalModulePathManager {
            file_path_configuration,
        }
    }

    pub fn resolve_to_interface_file_path(
        &self,
        external_module_path: &ein::ExternalUnresolvedModulePath,
    ) -> FilePath {
        FilePath::new(
            vec![
                self.file_path_configuration.output_directory().into(),
                EXTERNAL_PACKAGE_DIRECTORY.into(),
            ]
            .drain(..)
            .chain(external_module_path.components().iter().cloned())
            .collect(),
        )
        .with_extension(self.file_path_configuration.interface_file_extension())
    }

    pub fn convert_to_directory_path(&self, package_name: &str) -> FilePath {
        FilePath::new(
            vec![
                self.file_path_configuration.output_directory(),
                EXTERNAL_PACKAGE_DIRECTORY,
            ]
            .drain(..)
            .chain(package_name.split('/'))
            .map(String::from)
            .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_from_file_path() {
        assert_eq!(
            ExternalModulePathManager::new(&FilePathConfiguration::new("target", "", "", "json"))
                .resolve_to_interface_file_path(&ein::ExternalUnresolvedModulePath::new(vec![
                    "package".into(),
                    "Foo".into()
                ])),
            FilePath::new(vec![
                "target".into(),
                "packages".into(),
                "package".into(),
                "Foo.json".into()
            ])
        );
    }
}
