use super::error::CompileError;
use crate::ast::Module;

pub trait Pass {
    fn compile(&mut self, module: &Module) -> Result<Module, CompileError>;
}
