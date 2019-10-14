mod argument_omission;
mod main_function_name;
mod non_variable_application;

use crate::ast::*;
use argument_omission::*;
use main_function_name::*;
use non_variable_application::*;

pub fn desugar_without_types(module: &Module) -> Module {
    desugar_non_variable_applications(&desugar_main_function_name(module))
}

pub fn desugar_with_types(module: &Module) -> Module {
    desugar_argument_omission(module)
}
