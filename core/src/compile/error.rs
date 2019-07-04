use super::super::verify::VerificationError;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum CompileError {
    VariableNotFound,
    Verification,
}

impl Display for CompileError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "{:?}", self)
    }
}

impl Error for CompileError {}

impl From<VerificationError> for CompileError {
    fn from(_: VerificationError) -> Self {
        CompileError::Verification
    }
}
