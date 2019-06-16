use std::error::Error;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub struct TypeCheckError;

impl Display for TypeCheckError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "type check error")
    }
}

impl Error for TypeCheckError {
    fn description(&self) -> &str {
        "type check error"
    }
}
