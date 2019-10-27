mod file_storage;
mod linker;
mod raw_package_configuration;

pub use file_storage::*;
pub use linker::*;
use raw_package_configuration::*;

pub fn parse_package_configuration(
    source: &str,
) -> Result<app::PackageConfiguration, Box<dyn std::error::Error>> {
    let configuration: RawPackageConfiguration = serde_json::from_str(source)?;

    Ok(app::PackageConfiguration::new(
        configuration.name().into(),
        semver::Version::parse(configuration.version())?,
    ))
}
