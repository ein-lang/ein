use super::type_inference::TypeInferenceError;
use std::error::Error;
use std::fmt::Display;
use std::io;

#[derive(Debug, PartialEq)]
pub enum CompileError {
    External(ExternalCompileError),
    Internal(InternalCompileError),
}

impl Display for CompileError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            CompileError::External(error) => write!(
                formatter,
                "{}\n\nCompileError: Failed to compile due to the reason above",
                error
            ),
            CompileError::Internal(error) => write!(formatter, "CompileError: {}", error),
        }
    }
}

impl Error for CompileError {}

impl From<core::compile::CompileError> for CompileError {
    fn from(error: core::compile::CompileError) -> Self {
        CompileError::External(error.into())
    }
}

impl From<io::Error> for CompileError {
    fn from(error: io::Error) -> Self {
        CompileError::External(error.into())
    }
}

impl From<TypeInferenceError> for CompileError {
    fn from(error: TypeInferenceError) -> Self {
        CompileError::External(error.into())
    }
}

#[derive(Debug)]
pub enum ExternalCompileError {
    CoreCompileError(core::compile::CompileError),
    IOError(io::Error),
    TypeInferenceError(TypeInferenceError),
}

impl Display for ExternalCompileError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            ExternalCompileError::CoreCompileError(error) => {
                write!(formatter, "CoreCompileError: {}", error)
            }
            ExternalCompileError::IOError(error) => write!(formatter, "IOError: {}", error),
            ExternalCompileError::TypeInferenceError(error) => write!(formatter, "{}", error),
        }
    }
}

impl PartialEq for ExternalCompileError {
    fn eq(&self, error: &Self) -> bool {
        match (self, error) {
            (
                ExternalCompileError::CoreCompileError(_),
                ExternalCompileError::CoreCompileError(_),
            ) => true,
            (ExternalCompileError::IOError(_), ExternalCompileError::IOError(_)) => true,
            (
                ExternalCompileError::TypeInferenceError(_),
                ExternalCompileError::TypeInferenceError(_),
            ) => true,
            _ => false,
        }
    }
}

impl From<core::compile::CompileError> for ExternalCompileError {
    fn from(error: core::compile::CompileError) -> Self {
        ExternalCompileError::CoreCompileError(error)
    }
}

impl From<io::Error> for ExternalCompileError {
    fn from(error: io::Error) -> Self {
        ExternalCompileError::IOError(error)
    }
}

impl From<TypeInferenceError> for ExternalCompileError {
    fn from(error: TypeInferenceError) -> Self {
        ExternalCompileError::TypeInferenceError(error)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum InternalCompileError {
    MixedDefinitionsInLet,
}

impl Display for InternalCompileError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            InternalCompileError::MixedDefinitionsInLet => write!(
                formatter,
                "Cannot mix function and value definitions in a let expression",
            ),
        }
    }
}
