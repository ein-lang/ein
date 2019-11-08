mod file_storage;
mod linker;
mod package_configuration;

pub use file_storage::*;
pub use linker::*;
use package_configuration::*;

pub fn parse_package_configuration(
    source: &str,
) -> Result<ein::Package, Box<dyn std::error::Error>> {
    let configuration = serde_json::from_str::<PackageConfiguration>(source)?;

    Ok(ein::Package::new(
        configuration.name(),
        semver::Version::parse(configuration.version())?,
    ))
}
