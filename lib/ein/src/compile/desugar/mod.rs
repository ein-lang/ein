mod argument_omission;
mod record_update;

use crate::ast::*;
use argument_omission::*;
use record_update::*;

pub fn desugar_without_types(module: &Module) -> Module {
    desugar_record_update(module)
}

pub fn desugar_with_types(module: &Module) -> Module {
    desugar_argument_omission(module)
}
