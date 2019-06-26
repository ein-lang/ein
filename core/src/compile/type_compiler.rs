use super::llvm;
use crate::ast;
use crate::types::{self, Type};

pub struct TypeCompiler {}

impl TypeCompiler {
    pub fn new() -> Self {
        Self {}
    }

    pub unsafe fn compile(&self, type_: &Type) -> llvm::Type {
        match type_ {
            Type::Function(function) => llvm::Type::pointer(self.compile_unsized_closure(function)),
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

    pub unsafe fn compile_closure(
        &self,
        function_definition: &ast::FunctionDefinition,
    ) -> llvm::Type {
        llvm::Type::struct_(
            &vec![llvm::Type::pointer(
                self.compile_function(function_definition.type_()),
            )]
            .into_iter()
            .chain(
                function_definition
                    .environment()
                    .iter()
                    .map(|argument| self.compile(argument.type_())),
            )
            .collect::<Vec<_>>(),
        )
    }

    unsafe fn compile_unsized_closure(&self, function: &types::Function) -> llvm::Type {
        llvm::Type::struct_(&[llvm::Type::pointer(self.compile_function(function))])
    }
}
