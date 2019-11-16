use super::error::InfrastructureError;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PackageTarget {
    #[serde(rename(deserialize = "type"))]
    type_: app::TargetType,
    name: Option<String>,
}

impl PackageTarget {
    pub fn type_(&self) -> app::TargetType {
        self.type_
    }

    pub fn verify(&self) -> Result<(), InfrastructureError> {
        if self.type_ == app::TargetType::Command
            && (self.name.is_none() || self.name == Some("".into()))
        {
            Err(InfrastructureError::ConfigurationVerification(
                "command name required for command target".into(),
            ))
        } else if self.type_ == app::TargetType::Library && self.name.is_some() {
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
        serde_json::from_str::<PackageTarget>(r#"{ "type": "Command", "name": "foo" }"#)
            .unwrap()
            .verify()
            .unwrap();
    }

    #[test]
    fn parse_package_configuration_of_library_target() {
        serde_json::from_str::<PackageTarget>(r#"{ "type": "Library" }"#)
            .unwrap()
            .verify()
            .unwrap();
    }

    #[test]
    fn verify_no_name_field_for_binary_target() {
        let package_configuration =
            serde_json::from_str::<PackageTarget>(r#"{ "type": "Command" }"#).unwrap();

        assert!(package_configuration.verify().is_err());
    }

    #[test]
    fn verify_empty_name_field_for_binary_target() {
        let package_configuration =
            serde_json::from_str::<PackageTarget>(r#"{ "type": "Command", "name": "" }"#).unwrap();

        assert!(package_configuration.verify().is_err());
    }

    #[test]
    fn verify_no_name_field_for_library_target() {
        let package_configuration =
            serde_json::from_str::<PackageTarget>(r#"{ "type": "Library", "name": "foo" }"#)
                .unwrap();

        assert!(package_configuration.verify().is_err());
    }
}
