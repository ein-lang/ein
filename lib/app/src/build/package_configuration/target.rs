use super::error::PackageConfigurationError;
use super::target_type::TargetType;
use crate::build;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

#[derive(Deserialize, Serialize)]
pub struct Target {
    #[serde(rename(deserialize = "type"))]
    type_: TargetType,
    name: Option<String>,
}

impl TryInto<build::Target> for &Target {
    type Error = PackageConfigurationError;

    fn try_into(self) -> Result<build::Target, PackageConfigurationError> {
        match self.type_ {
            TargetType::Command => match &self.name {
                Some(name) => {
                    if name == "" {
                        Err(PackageConfigurationError::MissingCommandName)
                    } else {
                        Ok(build::Target::Command(build::CommandTarget::new(name)))
                    }
                }
                None => Err(PackageConfigurationError::MissingCommandName),
            },
            TargetType::Library => {
                if self.name.is_some() {
                    Err(PackageConfigurationError::CommandNameForLibrary)
                } else {
                    Ok(build::Target::Library)
                }
            }
        }
    }
}

impl TryInto<build::Target> for Target {
    type Error = PackageConfigurationError;

    fn try_into(self) -> Result<build::Target, PackageConfigurationError> {
        (&self).try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_package_configuration_of_binary_target() {
        let _: build::Target =
            serde_json::from_str::<Target>(r#"{ "type": "Command", "name": "foo" }"#)
                .unwrap()
                .try_into()
                .unwrap();
    }

    #[test]
    fn parse_package_configuration_of_library_target() {
        let _: build::Target = serde_json::from_str::<Target>(r#"{ "type": "Library" }"#)
            .unwrap()
            .try_into()
            .unwrap();
    }

    #[test]
    fn verify_no_name_field_for_binary_target() {
        let target: Result<build::Target, _> =
            serde_json::from_str::<Target>(r#"{ "type": "Command" }"#)
                .unwrap()
                .try_into();

        assert!(target.is_err());
    }

    #[test]
    fn verify_empty_name_field_for_binary_target() {
        let target: Result<build::Target, _> =
            serde_json::from_str::<Target>(r#"{ "type": "Command", "name": "" }"#)
                .unwrap()
                .try_into();

        assert!(target.is_err());
    }

    #[test]
    fn verify_no_name_field_for_library_target() {
        let target: Result<build::Target, _> =
            serde_json::from_str::<Target>(r#"{ "type": "Library", "name": "foo" }"#)
                .unwrap()
                .try_into();

        assert!(target.is_err());
    }
}
