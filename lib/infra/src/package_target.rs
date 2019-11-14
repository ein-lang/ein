use super::error::InfrastructureError;
use super::package_target_type::PackageTargetType;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PackageTarget {
    #[serde(rename(deserialize = "type"))]
    type_: PackageTargetType,
    #[serde(rename(deserialize = "exposedModules"))]
    exposed_modules: Option<Vec<String>>,
}

impl PackageTarget {
    pub fn type_(&self) -> PackageTargetType {
        self.type_
    }

    #[allow(dead_code)]
    pub fn exposed_modules(&self) -> &Option<Vec<String>> {
        &self.exposed_modules
    }

    pub fn verify(&self) -> Result<(), InfrastructureError> {
        if self.type_ == PackageTargetType::Command && self.exposed_modules.is_some() {
            Err(InfrastructureError::ConfigurationVerification(
                "no exposed modules allowed for binary target".into(),
            ))
        } else if self.type_ == PackageTargetType::Library
            && (self.exposed_modules == None || self.exposed_modules == Some(vec![]))
        {
            Err(InfrastructureError::ConfigurationVerification(
                "exposed modules needed for library target".into(),
            ))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_package_configuration_of_binary_target() {
        serde_json::from_str::<PackageTarget>(r#"{ "type": "Command" }"#)
            .unwrap()
            .verify()
            .unwrap();
    }

    #[test]
    fn parse_package_configuration_of_library_target() {
        serde_json::from_str::<PackageTarget>(
            r#"{ "type": "Library", "exposedModules": ["Main"] }"#,
        )
        .unwrap()
        .verify()
        .unwrap();
    }

    #[test]
    fn verify_no_exposed_modules_field_for_binary_target() {
        let package_configuration =
            serde_json::from_str::<PackageTarget>(r#"{ "type": "Command", "exposedModules": [] }"#)
                .unwrap();

        assert!(package_configuration.verify().is_err());
    }

    #[test]
    fn verify_no_exposed_modules_for_binary_target() {
        let package_configuration = serde_json::from_str::<PackageTarget>(
            r#"{ "type": "Command", "exposedModules": ["Main"] }"#,
        )
        .unwrap();

        assert!(package_configuration.verify().is_err());
    }

    #[test]
    fn verify_exposed_modules_field_for_library_target() {
        let package_configuration =
            serde_json::from_str::<PackageTarget>(r#"{ "type": "Library" }"#).unwrap();

        assert!(package_configuration.verify().is_err());
    }

    #[test]
    fn verify_exposed_modules_for_library_target() {
        let package_configuration =
            serde_json::from_str::<PackageTarget>(r#"{ "type": "Library", "exposedModules": [] }"#)
                .unwrap();

        assert!(package_configuration.verify().is_err());
    }
}
