mod argument_omission;
mod main_function_name;

use crate::ast::*;
use argument_omission::*;
use main_function_name::*;

pub fn desugar_without_types(module: &Module) -> Module {
    desugar_main_function_name(module)
}

pub fn desugar_with_types(module: &Module) -> Module {
    desugar_argument_omission(module)
}
