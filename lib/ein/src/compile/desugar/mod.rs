mod argument_omission;
mod record_update;

use super::error::CompileError;
use crate::ast::*;
use argument_omission::*;
use record_update::*;

pub fn desugar_without_types(module: &Module) -> Result<Module, CompileError> {
    desugar_record_update(module)
}

pub fn desugar_with_types(module: &Module) -> Result<Module, CompileError> {
    desugar_argument_omission(module)
}
