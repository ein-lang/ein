use super::llvm;
use crate::ast;
use crate::types::{self, Type};

pub struct TypeCompiler {
    context: llvm::Context,
    struct_types: Vec<llvm::Type>,
}

impl TypeCompiler {
    pub fn new() -> Self {
        Self {
            context: llvm::Context::new(),
            struct_types: vec![],
        }
    }

    pub fn compile(&self, type_: &Type) -> llvm::Type {
        match type_ {
            Type::Function(function) => llvm::Type::pointer(self.compile_unsized_closure(function)),
            Type::Index(index) => llvm::Type::pointer(self.struct_types[*index]),
            Type::Value(value) => self.compile_value(value),
        }
    }

    pub fn compile_value(&self, value: &types::Value) -> llvm::Type {
        match value {
            types::Value::Number => llvm::Type::double(),
        }
    }

    pub fn compile_entry_function(&self, function: &types::Function) -> llvm::Type {
        let mut arguments = vec![llvm::Type::pointer(Self::compile_unsized_environment())];

        arguments.extend_from_slice(
            &function
                .arguments()
                .iter()
                .map(|type_| self.compile(type_))
                .collect::<Vec<_>>(),
        );

        llvm::Type::function(self.compile_value(function.result()), &arguments)
    }

    pub fn compile_closure(&self, function_definition: &ast::FunctionDefinition) -> llvm::Type {
        let other =
            self.push_struct_type(self.compile_unsized_closure(function_definition.type_()));

        llvm::Type::struct_(&[
            llvm::Type::pointer(other.compile_entry_function(function_definition.type_())),
            other.compile_environment(function_definition.environment()),
        ])
    }

    pub fn compile_unsized_closure(&self, function: &types::Function) -> llvm::Type {
        let type_ = llvm::Type::struct_create_named(&function.to_id(), &self.context);

        type_.struct_set_body(&[
            llvm::Type::pointer(
                self.push_struct_type(type_)
                    .compile_entry_function(function),
            ),
            Self::compile_unsized_environment(),
        ]);

        type_
    }

    pub fn compile_environment(&self, free_variables: &[ast::Argument]) -> llvm::Type {
        llvm::Type::struct_(
            &free_variables
                .iter()
                .map(|argument| self.compile(argument.type_()))
                .collect::<Vec<_>>(),
        )
    }

    fn push_struct_type(&self, type_: llvm::Type) -> Self {
        Self {
            context: self.context.clone(),
            struct_types: self.struct_types.iter().chain(&[type_]).cloned().collect(),
        }
    }

    fn compile_unsized_environment() -> llvm::Type {
        llvm::Type::struct_(&[])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_number() {
        TypeCompiler::new().compile(&types::Value::Number.into());
    }

    #[test]
    fn compile_function() {
        TypeCompiler::new().compile(
            &types::Function::new(vec![types::Value::Number.into()], types::Value::Number).into(),
        );
    }

    #[test]
    fn compile_recursive_function_type() {
        TypeCompiler::new()
            .compile(&types::Function::new(vec![Type::Index(0)], types::Value::Number).into());
    }

    #[test]
    fn compile_function_twice() {
        let compiler = TypeCompiler::new();
        let type_ =
            types::Function::new(vec![types::Value::Number.into()], types::Value::Number).into();

        compiler.compile(&type_);
        compiler.compile(&type_);
    }
}
