mod partial_application_desugarer;
mod record_update_desugarer;

use super::error::CompileError;
use crate::ast::*;
use partial_application_desugarer::PartialApplicationDesugarer;
use record_update_desugarer::RecordUpdateDesugarer;

pub fn desugar_without_types(module: &Module) -> Result<Module, CompileError> {
    RecordUpdateDesugarer::new().desugar(module)
}

pub fn desugar_with_types(module: &Module) -> Result<Module, CompileError> {
    PartialApplicationDesugarer::new().desugar(module)
}
