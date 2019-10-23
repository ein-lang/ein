use std::error::Error;
use std::fmt::Display;
use std::io;

#[derive(Debug)]
pub enum RepositoryError {
    IO(std::io::Error),
    Serde(serde_json::Error),
}

impl Display for RepositoryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            RepositoryError::IO(error) => write!(formatter, "{}", error),
            RepositoryError::Serde(error) => write!(formatter, "{}", error),
        }
    }
}

impl Error for RepositoryError {}

impl From<io::Error> for RepositoryError {
    fn from(error: io::Error) -> Self {
        RepositoryError::IO(error)
    }
}

impl From<serde_json::Error> for RepositoryError {
    fn from(error: serde_json::Error) -> Self {
        RepositoryError::Serde(error)
    }
}
