mod json_application_build_configuration;
mod json_build_configuration;
mod json_external_package_configuration;
mod json_system_package_configuration;

use self::json_build_configuration::JsonBuildConfiguration;
use crate::common::BuildConfiguration;

pub fn serialize_build_configuration(
    build_configuration: &BuildConfiguration,
) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(&JsonBuildConfiguration::serialize(build_configuration))
}

pub fn deserialize_build_configuration(
    string: &str,
) -> Result<BuildConfiguration, serde_json::Error> {
    Ok(serde_json::from_str::<JsonBuildConfiguration>(string)?.deserialize())
}
