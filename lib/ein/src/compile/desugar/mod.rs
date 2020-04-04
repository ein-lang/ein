mod argument_omission;
mod non_variable_application;

use crate::ast::*;
use argument_omission::*;
use non_variable_application::*;

pub fn desugar_without_types(module: &Module) -> Module {
    desugar_non_variable_applications(&module)
}

pub fn desugar_with_types(module: &Module) -> Module {
    desugar_argument_omission(module)
}
