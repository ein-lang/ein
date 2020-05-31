use super::error::BuildError;
use super::package_configuration::{ExternalPackage, PackageConfiguration};
use super::package_interface::PackageInterface;
use super::path::FilePathManager;
use crate::infra::{ExternalPackageBuilder, ExternalPackageDownloader, FilePath, FileStorage};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

type ExternalModuleInterfaces = HashMap<ein::ExternalUnresolvedModulePath, ein::ModuleInterface>;

pub struct ExternalPackageInitializer<'a> {
    external_package_downloader: &'a dyn ExternalPackageDownloader,
    external_package_builder: &'a dyn ExternalPackageBuilder,
    file_storage: &'a dyn FileStorage,
    file_path_manager: &'a FilePathManager<'a>,
}

impl<'a> ExternalPackageInitializer<'a> {
    pub fn new(
        external_package_downloader: &'a dyn ExternalPackageDownloader,
        external_package_builder: &'a dyn ExternalPackageBuilder,
        file_storage: &'a dyn FileStorage,
        file_path_manager: &'a FilePathManager<'a>,
    ) -> Self {
        Self {
            external_package_downloader,
            external_package_builder,
            file_storage,
            file_path_manager,
        }
    }

    pub fn initialize(
        &self,
        package_configuration: &PackageConfiguration,
    ) -> Result<(Vec<FilePath>, ExternalModuleInterfaces), Box<dyn std::error::Error>> {
        let mut object_file_paths = vec![];
        let mut module_interfaces = HashMap::new();

        for (name, external_package) in package_configuration.dependencies() {
            let directory_path = self.generate_directory_path(name, external_package);

            if !self.file_storage.exists(&directory_path) {
                self.initialize_external_package(name, external_package, &directory_path)?;
            }

            object_file_paths.push(
                directory_path.join(&FilePath::new(&[self
                    .file_path_manager
                    .configuration()
                    .package_object_filename()])),
            );

            module_interfaces.extend(
                serde_json::from_str::<PackageInterface>(
                    &self.file_storage.read_to_string(
                        &directory_path.join(&FilePath::new(&[self
                            .file_path_manager
                            .configuration()
                            .package_interface_filename()])),
                    )?,
                )?
                .modules()
                .iter()
                .map(|module_interface| {
                    (
                        module_interface.path().external_unresolved(),
                        module_interface.clone(),
                    )
                }),
            );
        }

        Ok((object_file_paths, module_interfaces))
    }

    fn initialize_external_package(
        &self,
        name: &str,
        external_package: &ExternalPackage,
        directory_path: &FilePath,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.external_package_downloader.download(
            name,
            external_package.version(),
            &directory_path,
        )?;

        if !self.file_storage.exists(
            &directory_path.join(&FilePath::new(&[self
                .file_path_manager
                .configuration()
                .package_configuration_filename()])),
        ) {
            return Err(BuildError::ExternalPackageConfigurationFileNotFound {
                package_name: name.into(),
            }
            .into());
        }

        self.external_package_builder.build(&directory_path)?;

        Ok(())
    }

    fn generate_directory_path(&self, name: &str, external_package: &ExternalPackage) -> FilePath {
        let mut hasher = DefaultHasher::new();

        name.hash(&mut hasher);
        external_package.hash(&mut hasher);

        self.file_path_manager
            .configuration()
            .external_package_directory()
            .join(&FilePath::new(&[&format!("{:x}", hasher.finish())]))
    }
}

#[cfg(test)]
mod tests {
    use super::super::package_configuration::{ExternalPackage, PackageConfiguration, Target};
    use super::super::{ExternalPackageInitializer, FilePathConfiguration};
    use super::*;
    use crate::infra::{
        ExternalPackageBuilderFake, ExternalPackageDownloaderFake, FilePath, FileStorageFake,
    };

    #[test]
    fn new() {
        let file_path_configuration = FilePathConfiguration::new(
            "",
            "",
            "",
            "",
            "",
            FilePath::new(&["target"]),
            FilePath::new(&["target", "packages"]),
        );
        let file_storage = FileStorageFake::new(Default::default());

        ExternalPackageInitializer::new(
            &ExternalPackageDownloaderFake::new(Default::default(), &file_storage),
            &ExternalPackageBuilderFake::new(&file_path_configuration, &file_storage),
            &file_storage,
            &FilePathManager::new(&file_path_configuration),
        );
    }

    #[test]
    fn initialize_external_package() {
        let file_path_configuration = FilePathConfiguration::new(
            "ein.json",
            "",
            "",
            "",
            "",
            FilePath::new(&["target"]),
            FilePath::new(&["target", "packages"]),
        );
        let file_storage = FileStorageFake::new(Default::default());

        ExternalPackageInitializer::new(
            &ExternalPackageDownloaderFake::new(
                vec![(
                    "package".into(),
                    vec![(
                        FilePath::new(&["ein.json"]),
                        r#"{"target":{"type":"Library"},"dependencies":{}}"#.into(),
                    )]
                    .drain(..)
                    .collect(),
                )]
                .drain(..)
                .collect(),
                &file_storage,
            ),
            &ExternalPackageBuilderFake::new(&file_path_configuration, &file_storage),
            &file_storage,
            &FilePathManager::new(&file_path_configuration),
        )
        .initialize(&PackageConfiguration::new(
            Target::Library,
            vec![("package".into(), ExternalPackage::new("version"))]
                .drain(..)
                .collect(),
        ))
        .unwrap();
    }

    #[test]
    fn fail_to_initialize_external_package_without_package_configuration_file() {
        let file_path_configuration = FilePathConfiguration::new(
            "ein.json",
            "",
            "",
            "",
            "",
            FilePath::new(&["target"]),
            FilePath::new(&["target", "packages"]),
        );
        let file_storage = FileStorageFake::new(Default::default());

        let result = ExternalPackageInitializer::new(
            &ExternalPackageDownloaderFake::new(
                vec![("package".into(), Default::default())]
                    .drain(..)
                    .collect(),
                &file_storage,
            ),
            &ExternalPackageBuilderFake::new(&file_path_configuration, &file_storage),
            &file_storage,
            &FilePathManager::new(&file_path_configuration),
        )
        .initialize(&PackageConfiguration::new(
            Target::Library,
            vec![("package".into(), ExternalPackage::new("version"))]
                .drain(..)
                .collect(),
        ));

        assert_eq!(
            result.unwrap_err().downcast_ref(),
            Some(&BuildError::ExternalPackageConfigurationFileNotFound {
                package_name: "package".into()
            })
        )
    }
}
