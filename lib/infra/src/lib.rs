mod dependency_package;
mod error;
mod file_storage;
mod linker;
mod package_configuration;
mod target;
mod target_type;

use error::InfrastructureError;
pub use file_storage::*;
pub use linker::*;
use package_configuration::*;

pub fn parse_package_configuration(
    source: &str,
) -> Result<PackageConfiguration, InfrastructureError> {
    Ok(serde_json::from_str::<PackageConfiguration>(source)?)
}

pub fn get_package() -> Result<ein::Package, InfrastructureError> {
    Ok(get_package_from_git().unwrap_or_else(|_| ein::Package::new("main", "current")))
}

fn get_package_from_git() -> Result<ein::Package, InfrastructureError> {
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
        .join("/"),
        format!("{}", object.id()),
    ))
}
