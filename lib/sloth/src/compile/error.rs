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
            CompileError::CircularInitialization => {
                write!(formatter, "CompileError: Circular initialization detected",)
            }
            CompileError::CoreCompile(error) => write!(formatter, "CoreCompileError: {}", error),
            CompileError::MixedDefinitionsInLet(source_information) => write!(
                formatter,
                "CompileError: Cannot mix function and value definitions in a let expression\n{}",
                source_information
            ),
            CompileError::TypeInference(error) => write!(formatter, "{}", error),
        }
    }
}

impl Error for CompileError {}

impl From<core::compile::CompileError> for CompileError {
    fn from(error: core::compile::CompileError) -> Self {
        match error {
            core::compile::CompileError::CircularInitialization => {
                CompileError::CircularInitialization
            }
            _ => CompileError::CoreCompile(error),
        }
    }
}

impl From<TypeInferenceError> for CompileError {
    fn from(error: TypeInferenceError) -> Self {
        CompileError::TypeInference(error)
    }
}
