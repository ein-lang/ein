mod argument_omission;

use crate::ast::*;
use argument_omission::*;

// TODO Consider deleting this function.
pub fn desugar_without_types(module: &Module) -> Module {
    module.clone()
}

pub fn desugar_with_types(module: &Module) -> Module {
    desugar_argument_omission(module)
}
