use super::error::CompileError;
use crate::ast::Module;

pub trait Pass {
    fn compile(module: &Module) -> Result<Module, CompileError>;
}
