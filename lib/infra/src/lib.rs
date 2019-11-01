mod file_storage;
mod linker;
mod package_configuration;

pub use file_storage::*;
pub use linker::*;
use package_configuration::*;

pub fn parse_package_configuration(
    source: &str,
) -> Result<sloth::Package, Box<dyn std::error::Error>> {
    let configuration = serde_json::from_str::<PackageConfiguration>(source)?;

    Ok(sloth::Package::new(
        configuration.name().into(),
        semver::Version::parse(configuration.version())?,
    ))
}
