use crate::ast;
use crate::types::{self, Type};

pub struct TypeCompiler<'a> {
    context: &'a llvm::Context,
    module: &'a llvm::Module,
    struct_types: Vec<llvm::Type>,
}

impl<'a> TypeCompiler<'a> {
    pub fn new(context: &'a llvm::Context, module: &'a llvm::Module) -> Self {
        Self {
            context,
            module,
            struct_types: vec![],
        }
    }

    fn compile(&self, type_: &Type) -> llvm::Type {
        match type_ {
            Type::Function(function) => self
                .context
                .pointer_type(self.compile_unsized_closure(function)),
            Type::Index(index) => self.context.pointer_type(self.struct_types[*index]),
            Type::Value(value) => self.compile_value(value),
        }
    }

    pub fn compile_value(&self, value: &types::Value) -> llvm::Type {
        match value {
            types::Value::Number => self.context.double_type(),
        }
    }

    pub fn compile_closure(&self, function_definition: &ast::FunctionDefinition) -> llvm::Type {
        let other =
            self.push_struct_type(self.compile_unsized_closure(function_definition.type_()));

        self.context.struct_type(&[
            self.context
                .pointer_type(other.compile_entry_function(function_definition.type_())),
            other.compile_environment(function_definition.environment()),
        ])
    }

    pub fn compile_unsized_closure(&self, function: &types::Function) -> llvm::Type {
        let id = function.to_id();

        if let Some(type_) = self.module.get_type_by_name(&id) {
            return type_;
        }

        let type_ = self.context.named_struct_type(&id);

        type_.struct_set_body(&[
            self.context.pointer_type(
                self.push_struct_type(type_)
                    .compile_entry_function(function),
            ),
            self.compile_unsized_environment(),
        ]);

        type_
    }

    fn compile_environment(&self, free_variables: &[ast::Argument]) -> llvm::Type {
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

    fn compile_entry_function(&self, function: &types::Function) -> llvm::Type {
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

    fn push_struct_type(&self, type_: llvm::Type) -> Self {
        Self {
            context: self.context,
            module: self.module,
            struct_types: self.struct_types.iter().chain(&[type_]).cloned().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_number() {
        let context = llvm::Context::new();
        TypeCompiler::new(&context, &context.create_module(""))
            .compile(&types::Value::Number.into());
    }

    #[test]
    fn compile_function() {
        let context = llvm::Context::new();
        TypeCompiler::new(&context, &context.create_module("")).compile(
            &types::Function::new(vec![types::Value::Number.into()], types::Value::Number).into(),
        );
    }

    #[test]
    fn compile_recursive_function_type() {
        let context = llvm::Context::new();
        TypeCompiler::new(&context, &context.create_module(""))
            .compile(&types::Function::new(vec![Type::Index(0)], types::Value::Number).into());
    }

    #[test]
    fn compile_function_twice() {
        let context = llvm::Context::new();
        let module = context.create_module("");
        let compiler = TypeCompiler::new(&context, &module);
        let type_ =
            types::Function::new(vec![types::Value::Number.into()], types::Value::Number).into();

        compiler.compile(&type_);
        compiler.compile(&type_);
    }
}
