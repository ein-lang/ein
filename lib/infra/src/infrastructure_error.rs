use std::error::Error;
use std::fmt::Display;
use std::io;
use std::path::StripPrefixError;

#[derive(Debug, PartialEq)]
pub enum InfrastructureError {
    IO(String),
    NoParentDirectory,
    Serde(String),
    SemVer(semver::SemVerError),
    StripPrefix(StripPrefixError),
}

impl Display for InfrastructureError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::IO(message) => write!(formatter, "{}", message),
            Self::NoParentDirectory => write!(formatter, "no parent directory"),
            Self::SemVer(error) => write!(formatter, "{}", error),
            Self::Serde(message) => write!(formatter, "{}", message),
            Self::StripPrefix(error) => write!(formatter, "{}", error),
        }
    }
}

impl Error for InfrastructureError {}

impl From<io::Error> for InfrastructureError {
    fn from(error: io::Error) -> Self {
        Self::IO(format!("{}", error))
    }
}

impl From<semver::SemVerError> for InfrastructureError {
    fn from(error: semver::SemVerError) -> Self {
        Self::SemVer(error)
    }
}

impl From<serde_json::Error> for InfrastructureError {
    fn from(error: serde_json::Error) -> Self {
        Self::Serde(format!("{}", error))
    }
}

impl From<StripPrefixError> for InfrastructureError {
    fn from(error: StripPrefixError) -> Self {
        Self::StripPrefix(error)
    }
}
