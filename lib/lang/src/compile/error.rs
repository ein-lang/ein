use crate::ast;
use crate::debug::*;
use crate::path::ModulePath;
use crate::types;
use std::error::Error;
use std::fmt::Display;
use std::sync::Arc;

#[derive(Debug, PartialEq)]
pub enum CompileError {
    AnyEqualOperation(Arc<SourceInformation>),
    CaseArgumentTypeInvalid(Arc<SourceInformation>),
    DuplicateNames(Arc<SourceInformation>, Arc<SourceInformation>),
    ExportedNameNotFound { name: String },
    FunctionEqualOperation(Arc<SourceInformation>),
    FunctionExpected(Arc<SourceInformation>),
    MainFunctionNotFound(ModulePath),
    RecordEqualOperation(Arc<SourceInformation>),
    SsfCompile(ssf_llvm::CompileError),
    TypeNotFound(types::Reference),
    TypesNotMatched(Arc<SourceInformation>, Arc<SourceInformation>),
    TypeNotInferred(Arc<SourceInformation>),
    VariableNotFound(ast::Variable),
}

impl Display for CompileError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::AnyEqualOperation(source_information) => write!(
                formatter,
                "cannot compare Any type values\n{}",
                source_information
            ),
            Self::CaseArgumentTypeInvalid(source_information) => write!(
                formatter,
                "invalid argument type of case expression\n{}",
                source_information
            ),
            Self::ExportedNameNotFound { name } => {
                write!(formatter, "exported name \"{}\" not found", name)
            }
            Self::DuplicateNames(one, other) => {
                write!(formatter, "duplicate names\n{}\n{}", one, other)
            }
            Self::FunctionEqualOperation(source_information) => write!(
                formatter,
                "cannot compare functions\n{}",
                source_information
            ),
            Self::FunctionExpected(source_information) => {
                write!(formatter, "function expected\n{}", source_information)
            }
            Self::MainFunctionNotFound(path) => write!(
                formatter,
                "main function not found in main module {}",
                &path
            ),
            Self::RecordEqualOperation(source_information) => write!(
                formatter,
                "cannot compare records including functions or Any values\n{}",
                source_information
            ),
            Self::SsfCompile(error) => write!(formatter, "{}", error),
            Self::TypeNotFound(reference) => write!(
                formatter,
                "type \"{}\" not found\n{}",
                reference.name(),
                reference.source_information()
            ),
            Self::TypeNotInferred(source_information) => {
                write!(formatter, "failed to infer type\n{}", source_information)
            }
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

impl From<ssf_llvm::CompileError> for CompileError {
    fn from(error: ssf_llvm::CompileError) -> Self {
        Self::SsfCompile(error)
    }
}
