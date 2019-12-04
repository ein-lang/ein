use super::command_package_builder::CommandPackageBuilder;
use super::external_package_initializer::ExternalPackageInitializer;
use super::library_package_builder::LibraryPackageBuilder;
use super::package_configuration::{PackageConfiguration, Target};
use super::path::FilePathConfiguration;
use crate::infra::{
    Archiver, ExternalPackageBuilder, ExternalPackageDownloader, FilePath, FileStorage, Linker,
    Repository,
};

pub struct PackageBuilder<
    'a,
    R: Repository,
    S: FileStorage,
    L: Linker,
    A: Archiver,
    D: ExternalPackageDownloader,
    B: ExternalPackageBuilder,
> {
    command_package_builder: &'a CommandPackageBuilder<'a, S, L>,
    library_package_builder: &'a LibraryPackageBuilder<'a, S, A>,
    external_package_initializer: &'a ExternalPackageInitializer<'a, S, D, B>,
    repository: &'a R,
    file_storage: &'a S,
    file_path_configuration: &'a FilePathConfiguration,
}

impl<
        'a,
        R: Repository,
        S: FileStorage,
        L: Linker,
        A: Archiver,
        D: ExternalPackageDownloader,
        B: ExternalPackageBuilder,
    > PackageBuilder<'a, R, S, L, A, D, B>
{
    pub fn new(
        command_package_builder: &'a CommandPackageBuilder<'a, S, L>,
        library_package_builder: &'a LibraryPackageBuilder<'a, S, A>,
        external_package_initializer: &'a ExternalPackageInitializer<'a, S, D, B>,
        repository: &'a R,
        file_storage: &'a S,
        file_path_configuration: &'a FilePathConfiguration,
    ) -> Self {
        Self {
            command_package_builder,
            library_package_builder,
            external_package_initializer,
            repository,
            file_storage,
            file_path_configuration,
        }
    }

    pub fn build(&self) -> Result<(), Box<dyn std::error::Error>> {
        let package = self.repository.get_package()?;
        let package_configuration: PackageConfiguration = serde_json::from_str(
            &self.file_storage.read_to_string(&FilePath::new(&[self
                .file_path_configuration
                .package_configuration_filename()]))?,
        )?;

        self.external_package_initializer
            .initialize(&package_configuration)?;

        match package_configuration.target() {
            Target::Command(command_target) => self
                .command_package_builder
                .build(&package, command_target.name()),
            Target::Library => self.library_package_builder.build(&package),
        }
    }
}
