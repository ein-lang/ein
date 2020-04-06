mod partial_applications;
mod record_update;

use super::error::CompileError;
use crate::ast::*;
use partial_applications::*;
use record_update::*;

pub fn desugar_without_types(module: &Module) -> Result<Module, CompileError> {
    desugar_record_update(module)
}

pub fn desugar_with_types(module: &Module) -> Result<Module, CompileError> {
    PartialApplicationDesugarer::new().desugar(module)
}
