use super::super::verify::VerificationError;
use petgraph::algo::Cycle;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum CompileError {
    VariableNotFound,
    Verification,
    CircularInitialization,
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

impl<N> From<Cycle<N>> for CompileError {
    fn from(_: Cycle<N>) -> Self {
        Self::CircularInitialization
    }
}
