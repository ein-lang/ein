use super::type_check::TypeCheckError;
use std::error::Error;
use std::fmt::Display;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VerificationError {
    TypeCheck,
    InvalidFreeVariable,
}

impl Display for VerificationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "{:?}", self)
    }
}

impl Error for VerificationError {}

impl From<TypeCheckError> for VerificationError {
    fn from(_: TypeCheckError) -> Self {
        VerificationError::TypeCheck
    }
}
