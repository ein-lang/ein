use crate::{ast, debug::*, path::ModulePath, types};
use std::{error::Error, fmt::Display, sync::Arc};

#[derive(Debug, PartialEq)]
pub enum CompileError {
    AnyEqualOperation(Arc<SourceInformation>),
    CaseArgumentTypeInvalid(Arc<SourceInformation>),
    DuplicateNames(Arc<SourceInformation>, Arc<SourceInformation>),
    ExportedNameNotFound {
        name: String,
    },
    FunctionEqualOperation(Arc<SourceInformation>),
    FunctionExpected(Arc<SourceInformation>),
    MainFunctionNotFound(ModulePath),
    RecordElementNotFound {
        record_type: types::Record,
        name: String,
    },
    RecordEqualOperation(Arc<SourceInformation>),
    EirFmmCompile(eir_fmm::CompileError),
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
            Self::RecordElementNotFound { record_type, name } => write!(
                formatter,
                "element \"{}\" not found in record type\n{}",
                &name,
                record_type.source_information()
            ),
            Self::RecordEqualOperation(source_information) => write!(
                formatter,
                "cannot compare records including functions or Any values\n{}",
                source_information
            ),
            Self::EirFmmCompile(error) => {
                write!(formatter, "failed to compile eir to fmm: {:?}", error)
            }
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

impl From<eir_fmm::CompileError> for CompileError {
    fn from(error: eir_fmm::CompileError) -> Self {
        Self::EirFmmCompile(error)
    }
}
