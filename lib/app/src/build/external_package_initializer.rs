use super::error::BuildError;
use super::package_configuration::PackageConfiguration;
use super::path::{ExternalModulePathManager, FilePathConfiguration};
use crate::infra::{ExternalPackageBuilder, ExternalPackageDownloader, FilePath, FileStorage};

pub struct ExternalPackageInitializer<
    'a,
    S: FileStorage,
    D: ExternalPackageDownloader,
    B: ExternalPackageBuilder,
> {
    external_package_downloader: &'a D,
    external_package_builder: &'a B,
    external_module_path_manager: &'a ExternalModulePathManager<'a>,
    file_storage: &'a S,
    file_path_configuration: &'a FilePathConfiguration,
}

impl<'a, S: FileStorage, D: ExternalPackageDownloader, B: ExternalPackageBuilder>
    ExternalPackageInitializer<'a, S, D, B>
{
    pub fn new(
        external_package_downloader: &'a D,
        external_package_builder: &'a B,
        external_module_path_manager: &'a ExternalModulePathManager<'a>,
        file_storage: &'a S,
        file_path_configuration: &'a FilePathConfiguration,
    ) -> Self {
        Self {
            external_package_downloader,
            external_package_builder,
            external_module_path_manager,
            file_storage,
            file_path_configuration,
        }
    }

    pub fn initialize(
        &self,
        package_configuration: &PackageConfiguration,
    ) -> Result<Vec<FilePath>, Box<dyn std::error::Error>> {
        let mut object_file_paths = vec![];

        for (name, external_package) in package_configuration.dependencies() {
            let directory_path = self
                .external_module_path_manager
                .convert_to_directory_path(name);

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
                    .file_path_configuration
                    .package_configuration_filename()])),
            ) {
                return Err(BuildError::ExternalPackageConfigurationFileNotFound {
                    package_name: name.into(),
                }
                .into());
            }

            self.external_package_builder.build(&directory_path)?;
            object_file_paths.push(directory_path.join(&FilePath::new(&[
                self.file_path_configuration.package_object_filename(),
            ])))
        }

        Ok(object_file_paths)
    }
}

#[cfg(test)]
mod tests {
    use super::super::package_configuration::{ExternalPackage, PackageConfiguration, Target};
    use super::super::{ExternalPackageInitializer, FilePathConfiguration};
    use super::*;
    use crate::infra::{
        ExternalPackageBuilderStub, ExternalPackageDownloaderFake, FilePath, FileStorageFake,
    };

    #[test]
    fn new() {
        let file_path_configuration =
            FilePathConfiguration::new("", "", "", "", "", FilePath::new(&["target"]));
        let file_storage = FileStorageFake::new(Default::default());

        ExternalPackageInitializer::new(
            &ExternalPackageDownloaderFake::new(Default::default(), &file_storage),
            &ExternalPackageBuilderStub::new(),
            &ExternalModulePathManager::new(&file_path_configuration),
            &file_storage,
            &file_path_configuration,
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
            &ExternalPackageBuilderStub::new(),
            &ExternalModulePathManager::new(&file_path_configuration),
            &file_storage,
            &file_path_configuration,
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
            &ExternalPackageBuilderStub::new(),
            &ExternalModulePathManager::new(&file_path_configuration),
            &file_storage,
            &file_path_configuration,
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
