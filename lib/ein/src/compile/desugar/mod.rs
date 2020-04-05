mod record_update;
mod unsaturated_applications;

use super::error::CompileError;
use crate::ast::*;
use record_update::*;
use unsaturated_applications::*;

pub fn desugar_without_types(module: &Module) -> Result<Module, CompileError> {
    desugar_record_update(module)
}

pub fn desugar_with_types(module: &Module) -> Result<Module, CompileError> {
    desugar_unsaturated_applications(module)
}
