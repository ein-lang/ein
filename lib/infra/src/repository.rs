use super::error::InfrastructureError;

#[derive(Default)]
pub struct Repository;

impl Repository {
    pub fn new() -> Self {
        Self
    }

    fn get_package_from_git(&self) -> Result<ein::Package, Box<dyn std::error::Error>> {
        let repository = git2::Repository::discover(".")?;
        let url = url::Url::parse(
            repository
                .find_remote("origin")?
                .url()
                .ok_or(InfrastructureError::OriginUrlNotFound)?,
        )?;
        let object = repository.head()?.peel(git2::ObjectType::Any)?;

        Ok(ein::Package::new(
            [
                url.host_str().ok_or(InfrastructureError::HostNotFound)?,
                url.path(),
            ]
            .concat(),
            format!("{}", object.id()),
        ))
    }
}

impl app::Repository for Repository {
    fn get_package(&self) -> Result<ein::Package, Box<dyn std::error::Error>> {
        Ok(self
            .get_package_from_git()
            .unwrap_or_else(|_| ein::Package::new("main", "current")))
    }
}
