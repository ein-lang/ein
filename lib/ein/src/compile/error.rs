use super::type_inference::TypeInferenceError;
use crate::debug::*;
use std::error::Error;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum CompileError {
    CircularInitialization,
    CoreCompile(core::compile::CompileError),
    MixedDefinitionsInLet(Rc<SourceInformation>),
    TypeInference(TypeInferenceError),
}

impl Display for CompileError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::CircularInitialization => {
                write!(formatter, "CompileError: Circular initialization detected",)
            }
            Self::CoreCompile(error) => write!(formatter, "CoreCompileError: {}", error),
            Self::MixedDefinitionsInLet(source_information) => write!(
                formatter,
                "CompileError: Cannot mix function and value definitions in a let expression\n{}",
                source_information
            ),
            Self::TypeInference(error) => write!(formatter, "{}", error),
        }
    }
}

impl Error for CompileError {}

impl From<core::compile::CompileError> for CompileError {
    fn from(error: core::compile::CompileError) -> Self {
        match error {
            core::compile::CompileError::CircularInitialization => Self::CircularInitialization,
            _ => Self::CoreCompile(error),
        }
    }
}

impl From<TypeInferenceError> for CompileError {
    fn from(error: TypeInferenceError) -> Self {
        Self::TypeInference(error)
    }
}
