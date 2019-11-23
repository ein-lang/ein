use super::package_configuration::PackageConfiguration;

pub struct ExternalPackageInitializer<'a> {
    package_configuration: &'a PackageConfiguration,
    external_package_directory: std::path::PathBuf,
}

impl<'a> ExternalPackageInitializer<'a> {
    pub fn new(
        package_configuration: &'a PackageConfiguration,
        external_package_directory: impl AsRef<std::path::Path>,
    ) -> Self {
        Self {
            package_configuration,
            external_package_directory: external_package_directory.as_ref().into(),
        }
    }
}

impl<'a> app::ExternalPackageInitializer for ExternalPackageInitializer<'a> {
    fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        for (name, external_package) in self.package_configuration.dependencies() {
            let directory = self
                .external_package_directory
                .join(name.split('/').collect::<std::path::PathBuf>());

            if directory.exists() {
                continue;
            }

            let url = url::Url::parse(&["https://", name].concat())?;
            let repository = git2::Repository::clone(url.as_str(), directory)?;

            repository.checkout_tree(
                &repository
                    .resolve_reference_from_short_name(external_package.version())?
                    .peel(git2::ObjectType::Any)?,
                None,
            )?;
        }

        Ok(())
    }
}
