mod dependency_package;
mod file_storage;
mod linker;
mod package_configuration;

pub use file_storage::*;
pub use linker::*;
use package_configuration::*;

pub fn parse_package_configuration(
    source: &str,
) -> Result<PackageConfiguration, serde_json::Error> {
    serde_json::from_str::<PackageConfiguration>(source)
}
