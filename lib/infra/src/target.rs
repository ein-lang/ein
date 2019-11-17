use super::error::InfrastructureError;
use super::target_type::TargetType;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

#[derive(Deserialize, Serialize)]
pub struct Target {
    #[serde(rename(deserialize = "type"))]
    type_: TargetType,
    name: Option<String>,
}

impl Target {
    pub fn type_(&self) -> TargetType {
        self.type_
    }
}

impl TryInto<app::Target> for &Target {
    type Error = InfrastructureError;

    fn try_into(self) -> Result<app::Target, InfrastructureError> {
        match self.type_ {
            TargetType::Command => match &self.name {
                Some(name) => {
                    if name == "" {
                        Err(InfrastructureError::ConfigurationVerification(
                            "empty command name not allowed".into(),
                        ))
                    } else {
                        Ok(app::Target::Command(app::CommandTarget::new(name)))
                    }
                }
                None => Err(InfrastructureError::ConfigurationVerification(
                    "command name required for command target".into(),
                )),
            },
            TargetType::Library => match self.name {
                Some(_) => Err(InfrastructureError::ConfigurationVerification(
                    "exposed modules needed for library target".into(),
                )),
                None => Ok(app::Target::Library),
            },
        }
    }
}

impl TryInto<app::Target> for Target {
    type Error = InfrastructureError;

    fn try_into(self) -> Result<app::Target, InfrastructureError> {
        (&self).try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_package_configuration_of_binary_target() {
        let _: app::Target =
            serde_json::from_str::<Target>(r#"{ "type": "Command", "name": "foo" }"#)
                .unwrap()
                .try_into()
                .unwrap();
    }

    #[test]
    fn parse_package_configuration_of_library_target() {
        let _: app::Target = serde_json::from_str::<Target>(r#"{ "type": "Library" }"#)
            .unwrap()
            .try_into()
            .unwrap();
    }

    #[test]
    fn verify_no_name_field_for_binary_target() {
        let target: Result<app::Target, _> =
            serde_json::from_str::<Target>(r#"{ "type": "Command" }"#)
                .unwrap()
                .try_into();

        assert!(target.is_err());
    }

    #[test]
    fn verify_empty_name_field_for_binary_target() {
        let target: Result<app::Target, _> =
            serde_json::from_str::<Target>(r#"{ "type": "Command", "name": "" }"#)
                .unwrap()
                .try_into();

        assert!(target.is_err());
    }

    #[test]
    fn verify_no_name_field_for_library_target() {
        let target: Result<app::Target, _> =
            serde_json::from_str::<Target>(r#"{ "type": "Library", "name": "foo" }"#)
                .unwrap()
                .try_into();

        assert!(target.is_err());
    }
}
