use crate::ast;
use crate::types::{self, Type};

pub struct TypeCompiler<'a> {
    context: &'a llvm::Context,
}

impl<'a> TypeCompiler<'a> {
    pub fn new(context: &'a llvm::Context) -> Self {
        Self { context }
    }

    pub fn compile(&self, type_: &Type) -> llvm::Type {
        match type_ {
            Type::Function(function) => self
                .context
                .pointer_type(self.compile_unsized_closure(function)),
            Type::Value(value) => self.compile_value(value),
        }
    }

    pub fn compile_value(&self, value: &types::Value) -> llvm::Type {
        match value {
            types::Value::Number => self.context.double_type(),
        }
    }

    pub fn compile_entry_function(&self, function: &types::Function) -> llvm::Type {
        let mut arguments = vec![self
            .context
            .pointer_type(self.compile_unsized_environment())];

        arguments.extend_from_slice(
            &function
                .arguments()
                .iter()
                .map(|type_| self.compile(type_))
                .collect::<Vec<_>>(),
        );

        self.context
            .function_type(self.compile_value(function.result()), &arguments)
    }

    pub fn compile_closure(&self, function_definition: &ast::FunctionDefinition) -> llvm::Type {
        self.context.struct_type(&[
            self.context
                .pointer_type(self.compile_entry_function(function_definition.type_())),
            self.compile_environment(function_definition.environment()),
        ])
    }

    pub fn compile_unsized_closure(&self, function: &types::Function) -> llvm::Type {
        self.context.struct_type(&[
            self.context
                .pointer_type(self.compile_entry_function(function)),
            self.compile_unsized_environment(),
        ])
    }

    pub fn compile_environment(&self, free_variables: &[ast::Argument]) -> llvm::Type {
        self.context.struct_type(
            &free_variables
                .iter()
                .map(|argument| self.compile(argument.type_()))
                .collect::<Vec<_>>(),
        )
    }

    fn compile_unsized_environment(&self) -> llvm::Type {
        self.context.struct_type(&[])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_number() {
        TypeCompiler::new(&llvm::Context::new()).compile(&types::Value::Number.into());
    }

    #[test]
    fn compile_function() {
        TypeCompiler::new(&llvm::Context::new()).compile(
            &types::Function::new(vec![types::Value::Number.into()], types::Value::Number).into(),
        );
    }

    #[test]
    fn compile_function_twice() {
        let context = llvm::Context::new();
        let compiler = TypeCompiler::new(&context);
        let type_ =
            types::Function::new(vec![types::Value::Number.into()], types::Value::Number).into();

        compiler.compile(&type_);
        compiler.compile(&type_);
    }
}
