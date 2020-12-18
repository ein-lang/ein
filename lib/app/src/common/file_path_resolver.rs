use super::external_package::ExternalPackage;
use super::file_path::FilePath;
use super::file_path_configuration::FilePathConfiguration;
use super::static_file_path_manager::StaticFilePathManager;

pub struct FilePathResolver<'a> {
    static_file_path_manager: &'a StaticFilePathManager,
    file_path_configuration: &'a FilePathConfiguration,
}

impl<'a> FilePathResolver<'a> {
    pub fn new(
        static_file_path_manager: &'a StaticFilePathManager,
        file_path_configuration: &'a FilePathConfiguration,
    ) -> Self {
        Self {
            static_file_path_manager,
            file_path_configuration,
        }
    }

    pub fn resolve_to_source_file_path(
        &self,
        directory_path: &FilePath,
        internal_module_path: &ein::InternalUnresolvedModulePath,
    ) -> FilePath {
        directory_path.join(
            &FilePath::new(internal_module_path.components())
                .with_extension(&self.file_path_configuration.source_file_extension),
        )
    }

    pub fn resolve_object_file_path(
        &self,
        package_directroy: &FilePath,
        id: impl AsRef<str>,
    ) -> FilePath {
        package_directroy.join(
            &self
                .static_file_path_manager
                .object_directory_path()
                .join(&FilePath::new(&[&id]))
                .with_extension(&self.file_path_configuration.object_file_extension),
        )
    }

    pub fn resolve_interface_file_path(&self, package_directroy: &FilePath, id: &str) -> FilePath {
        self.resolve_object_file_path(package_directroy, id)
            .with_extension(&self.file_path_configuration.interface_file_extension)
    }

    pub fn resolve_to_module_path(
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
        self.static_file_path_manager
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
