use super::type_inference::TypeInferenceError;
use crate::debug::*;
use std::error::Error;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum CompileError {
    CircularInitialization,
    ExportedNameNotFound { name: String },
    MixedDefinitionsInLet(Rc<SourceInformation>),
    SsfAnalysis(ssf::AnalysisError),
    SsfCompile(ssf_llvm::CompileError),
    TypeInference(TypeInferenceError),
}

impl Display for CompileError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::CircularInitialization => {
                write!(formatter, "circular variable initialization detected",)
            }
            Self::ExportedNameNotFound { name } => {
                write!(formatter, "exported name \"{}\" not found", name)
            }
            Self::MixedDefinitionsInLet(source_information) => write!(
                formatter,
                "cannot mix function and value definitions in a let expression\n{}",
                source_information
            ),
            Self::SsfAnalysis(error) => write!(formatter, "{}", error),
            Self::SsfCompile(error) => write!(formatter, "{}", error),
            Self::TypeInference(error) => write!(formatter, "{}", error),
        }
    }
}

impl Error for CompileError {}

impl From<ssf::AnalysisError> for CompileError {
    fn from(error: ssf::AnalysisError) -> Self {
        match error {
            ssf::AnalysisError::CircularInitialization => Self::CircularInitialization,
            _ => Self::SsfAnalysis(error),
        }
    }
}

impl From<ssf_llvm::CompileError> for CompileError {
    fn from(error: ssf_llvm::CompileError) -> Self {
        Self::SsfCompile(error)
    }
}

impl From<TypeInferenceError> for CompileError {
    fn from(error: TypeInferenceError) -> Self {
        Self::TypeInference(error)
    }
}
