#[derive(Default)]
pub struct ExternalPackageDownloader;

impl ExternalPackageDownloader {
    pub fn new() -> Self {
        Self
    }
}

impl app::ExternalPackageDownloader for ExternalPackageDownloader {
    fn download(
        &self,
        external_package: &app::ExternalPackage,
        directory_path: &app::FilePath,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let directory_path = directory_path.components().collect::<std::path::PathBuf>();

        let url = url::Url::parse(&["https://", external_package.name()].concat())?;
        let repository = git2::Repository::clone(url.as_str(), &directory_path)?;

        repository.checkout_tree(
            &repository
                .resolve_reference_from_short_name(external_package.version())?
                .peel(git2::ObjectType::Any)?,
            None,
        )?;

        Ok(())
    }
}
