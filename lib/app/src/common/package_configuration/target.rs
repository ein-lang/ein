use super::command_target::CommandTarget;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Target {
    Command(CommandTarget),
    Library,
}

#[cfg(test)]
mod tests {
    use super::super::system_package_configuration::SystemPackageConfiguration;
    use super::*;

    #[test]
    fn parse_command_target() {
        assert_eq!(
            serde_json::from_str::<Target>(
                r#"{"type":"Command","name":"foo","systemPackage":{"name":"github.com/foo/bar","version":"baz"}}"#
            )
            .unwrap(),
            Target::Command(CommandTarget::new(
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
