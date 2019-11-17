mod dependency_package;
mod error;
mod file_storage;
mod linker;
mod package_configuration;
mod package_target;
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
