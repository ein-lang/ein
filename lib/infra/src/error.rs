use std::error::Error;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum InfrastructureError {
    HostNotFound,
    OriginUrlNotFound,
}

impl Error for InfrastructureError {}

impl Display for InfrastructureError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::HostNotFound => write!(formatter, "host name for package name not defined"),
            Self::OriginUrlNotFound => write!(formatter, "repository origin URL not defined"),
        }
    }
}
