use std::error::Error;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum InfrastructureError {
    ConfigurationParse(String),
    ConfigurationVerification(String),
    Git(git2::Error),
    HostNotFound,
    OriginUrlNotFound,
    Url(url::ParseError),
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
            Self::Git(error) => write!(formatter, "GitError: {}", error),
            Self::HostNotFound => write!(formatter, "HostNotFoundError"),
            Self::OriginUrlNotFound => write!(formatter, "OriginUrlNotFoundError"),
            Self::Url(error) => write!(formatter, "UrlError: {}", error),
        }
    }
}

impl Error for InfrastructureError {}

impl From<git2::Error> for InfrastructureError {
    fn from(error: git2::Error) -> Self {
        Self::Git(error)
    }
}

impl From<serde_json::Error> for InfrastructureError {
    fn from(error: serde_json::Error) -> Self {
        Self::ConfigurationParse(format!("{}", error))
    }
}

impl From<url::ParseError> for InfrastructureError {
    fn from(error: url::ParseError) -> Self {
        Self::Url(error)
    }
}
