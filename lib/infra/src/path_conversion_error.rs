use std::error::Error;
use std::fmt::Display;
use std::io;
use std::path::StripPrefixError;

#[derive(Debug)]
pub enum PathConversionError {
    IO(std::io::Error),
    StripPrefix(StripPrefixError),
}

impl Display for PathConversionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            PathConversionError::IO(error) => write!(formatter, "{}", error),
            PathConversionError::StripPrefix(error) => write!(formatter, "{}", error),
        }
    }
}

impl Error for PathConversionError {}

impl PartialEq for PathConversionError {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::IO(_) => match other {
                Self::IO(_) => true,
                Self::StripPrefix(_) => false,
            },
            Self::StripPrefix(error) => match other {
                Self::IO(_) => false,
                Self::StripPrefix(other) => error == other,
            },
        }
    }
}

impl From<io::Error> for PathConversionError {
    fn from(error: io::Error) -> Self {
        PathConversionError::IO(error)
    }
}

impl From<StripPrefixError> for PathConversionError {
    fn from(error: StripPrefixError) -> Self {
        PathConversionError::StripPrefix(error)
    }
}
