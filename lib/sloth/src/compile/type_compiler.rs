use crate::types::{self, Type};

pub struct TypeCompiler {}

impl TypeCompiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile(&self, type_: &Type) -> core::types::Type {
        match type_ {
            Type::Function(function) => self.compile_function(function).into(),
            Type::Number(_) => core::types::Value::Number.into(),
            Type::Variable(_) => unreachable!(),
        }
    }

    pub fn compile_function(&self, function: &types::Function) -> core::types::Function {
        core::types::Function::new(
            function
                .arguments()
                .iter()
                .map(|type_| self.compile(*type_))
                .collect::<Vec<_>>(),
            self.compile_value(function.last_result()),
        )
    }

    pub fn compile_value(&self, type_: &Type) -> core::types::Value {
        match type_ {
            Type::Function(_) => unreachable!(),
            Type::Number(_) => core::types::Value::Number,
            Type::Variable(_) => unreachable!(),
        }
    }
}
