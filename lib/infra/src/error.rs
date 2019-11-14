use std::error::Error;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum InfrastructureError {
    ConfigurationParse(String),
    ConfigurationVerification(String),
}

impl Display for InfrastructureError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::ConfigurationParse(message) => {
                write!(formatter, "ConfigurationParseError: {}", message)
            }
            Self::ConfigurationVerification(message) => {
                write!(formatter, "ConfigurationVerificationError: {}", message)
            }
        }
    }
}

impl Error for InfrastructureError {}

impl From<serde_json::Error> for InfrastructureError {
    fn from(error: serde_json::Error) -> Self {
        Self::ConfigurationParse(format!("{}", error))
    }
}
