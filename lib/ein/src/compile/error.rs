use crate::ast;
use crate::debug::*;
use crate::types;
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
    TypeNotFound(types::Reference),
    TypesNotMatched(Rc<SourceInformation>, Rc<SourceInformation>),
    VariableNotFound(ast::Variable),
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
            Self::TypeNotFound(reference) => write!(
                formatter,
                "type \"{}\" not found\n{}",
                reference.name(),
                reference.source_information()
            ),
            Self::TypesNotMatched(lhs_source_information, rhs_source_information) => write!(
                formatter,
                "types not matched\n{}\n{}",
                lhs_source_information, rhs_source_information
            ),
            Self::VariableNotFound(variable) => write!(
                formatter,
                "variable \"{}\" not found\n{}",
                variable.name(),
                variable.source_information()
            ),
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
