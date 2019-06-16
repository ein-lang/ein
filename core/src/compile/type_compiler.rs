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
            Type::Value(value) => self.compile_value(value),
        }
    }

    pub unsafe fn compile_value(&self, value: &types::Value) -> llvm::Type {
        match value {
            types::Value::Number => llvm::Type::double(),
        }
    }

    pub unsafe fn compile_function(&self, function: &types::Function) -> llvm::Type {
        let mut arguments = vec![llvm::Type::pointer(llvm::Type::i8())];

        arguments.extend_from_slice(
            &function
                .arguments()
                .iter()
                .map(|type_| self.compile(type_))
                .collect::<Vec<_>>(),
        );

        llvm::Type::function(self.compile_value(function.result()), &mut arguments)
    }
}
