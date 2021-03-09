use super::application_target::ApplicationTarget;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Target {
    Application(ApplicationTarget),
    Library,
}

#[cfg(test)]
mod tests {
    use super::super::system_package_configuration::SystemPackageConfiguration;
    use super::*;

    #[test]
    fn parse_application_target() {
        assert_eq!(
            serde_json::from_str::<Target>(
                r#"{"type":"Application","name":"foo","systemPackage":{"name":"github.com/foo/bar","version":"baz"}}"#
            )
            .unwrap(),
            Target::Application(ApplicationTarget::new(
                "foo",
                SystemPackageConfiguration::new("github.com/foo/bar", "baz")
            ))
        );
    }

    #[test]
    fn parse_library_target() {
        assert_eq!(
            serde_json::from_str::<Target>(r#"{"type":"Library"}"#).unwrap(),
            Target::Library
        );
    }
}
