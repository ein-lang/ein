use std::error::Error;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub struct TypeCheckError;

impl Display for TypeCheckError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "{:?}", self)
    }
}

impl Error for TypeCheckError {}
