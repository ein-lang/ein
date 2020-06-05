use crate::build::{ExternalPackage, FilePathConfiguration};
use crate::infra::FilePath;

pub struct FilePathManager<'a> {
    file_path_configuration: &'a FilePathConfiguration,
}

impl<'a> FilePathManager<'a> {
    pub fn new(file_path_configuration: &'a FilePathConfiguration) -> Self {
        Self {
            file_path_configuration,
        }
    }

    pub fn configuration(&self) -> &FilePathConfiguration {
        &self.file_path_configuration
    }

    pub fn resolve_to_source_file_path(
        &self,
        internal_module_path: &ein::InternalUnresolvedModulePath,
    ) -> FilePath {
        FilePath::new(internal_module_path.components())
            .with_extension(self.file_path_configuration.source_file_extension())
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

    pub fn resolve_to_external_package_directory_path(
        &self,
        external_package: &ExternalPackage,
    ) -> FilePath {
        self.file_path_configuration
            .external_packages_directory_path()
            .join(
                &external_package
                    .name()
                    .parse::<FilePath>()
                    .unwrap()
                    .join(&FilePath::new(&[external_package.version()])),
            )
    }
}
