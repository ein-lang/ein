use super::command_target::CommandTarget;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Target {
    Command(CommandTarget),
    Library,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_command_target() {
        assert_eq!(
            serde_json::from_str::<Target>(r#"{"type":"Command","name":"foo"}"#).unwrap(),
            Target::Command(CommandTarget::new("foo"))
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
