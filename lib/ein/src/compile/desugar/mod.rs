mod argument_omission;
mod non_variable_application;
mod record_element_operator;

use crate::ast::*;
use argument_omission::*;
use non_variable_application::*;
use record_element_operator::*;

pub fn desugar_without_types(module: &Module) -> Module {
    desugar_non_variable_applications(&desugar_record_element_operators(&module))
}

pub fn desugar_with_types(module: &Module) -> Module {
    desugar_argument_omission(module)
}
