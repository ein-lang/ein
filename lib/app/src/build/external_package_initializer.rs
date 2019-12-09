use super::error::BuildError;
use super::package_configuration::PackageConfiguration;
use super::package_interface::PackageInterface;
use super::path::FilePathManager;
use crate::infra::{ExternalPackageBuilder, ExternalPackageDownloader, FilePath, FileStorage};
use std::collections::HashMap;

type ExternalModuleInterfaces =
    HashMap<ein::ExternalUnresolvedModulePath, ein::ast::ModuleInterface>;

pub struct ExternalPackageInitializer<
    'a,
    S: FileStorage,
    D: ExternalPackageDownloader,
    B: ExternalPackageBuilder,
> {
    external_package_downloader: &'a D,
    external_package_builder: &'a B,
    file_path_manager: &'a FilePathManager<'a>,
    file_storage: &'a S,
}

impl<'a, S: FileStorage, D: ExternalPackageDownloader, B: ExternalPackageBuilder>
    ExternalPackageInitializer<'a, S, D, B>
{
    pub fn new(
        external_package_downloader: &'a D,
        external_package_builder: &'a B,
        file_path_manager: &'a FilePathManager,
        file_storage: &'a S,
    ) -> Self {
        Self {
            external_package_downloader,
            external_package_builder,
            file_path_manager,
            file_storage,
        }
    }

    pub fn initialize(
        &self,
        package_configuration: &PackageConfiguration,
    ) -> Result<(Vec<FilePath>, ExternalModuleInterfaces), Box<dyn std::error::Error>> {
        let mut object_file_paths = vec![];
        let mut module_interfaces = HashMap::new();

        for (name, external_package) in package_configuration.dependencies() {
            let directory_path = self.file_path_manager.convert_to_directory_path(name);

            if self.file_storage.exists(&directory_path) {
                continue;
            }

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
        let file_path_configuration =
            FilePathConfiguration::new("", "", "", "", "", FilePath::new(&["target"]));
        let file_storage = FileStorageFake::new(Default::default());

        ExternalPackageInitializer::new(
            &ExternalPackageDownloaderFake::new(Default::default(), &file_storage),
            &ExternalPackageBuilderFake::new(&file_path_configuration, &file_storage),
            &FilePathManager::new(&file_path_configuration),
            &file_storage,
        );
    }

    #[test]
    fn initialize_external_package() {
        let file_path_configuration =
            FilePathConfiguration::new("ein.json", "", "", "", "", FilePath::new(&["target"]));
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
            &FilePathManager::new(&file_path_configuration),
            &file_storage,
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
        let file_path_configuration =
            FilePathConfiguration::new("ein.json", "", "", "", "", FilePath::new(&["target"]));
        let file_storage = FileStorageFake::new(Default::default());

        let result = ExternalPackageInitializer::new(
            &ExternalPackageDownloaderFake::new(
                vec![("package".into(), Default::default())]
                    .drain(..)
                    .collect(),
                &file_storage,
            ),
            &ExternalPackageBuilderFake::new(&file_path_configuration, &file_storage),
            &FilePathManager::new(&file_path_configuration),
            &file_storage,
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
