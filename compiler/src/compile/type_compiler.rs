use super::llvm;
use crate::types::{self, Type};

pub struct TypeCompiler {}

impl TypeCompiler {
    pub fn new() -> Self {
        Self {}
    }

    pub unsafe fn compile(&self, type_: &Type) -> llvm::Type {
        match type_ {
            Type::Function(function) => self.compile_function(function),
            Type::Number => llvm::Type::double(),
        }
    }

    pub unsafe fn compile_function(&self, function: &types::Function) -> llvm::Type {
        llvm::Type::function(
            self.compile(function.result()),
            &mut function
                .arguments()
                .iter()
                .map(|type_| self.compile(type_))
                .collect::<Vec<llvm::Type>>(),
        )
    }
}
