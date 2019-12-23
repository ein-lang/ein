use super::reference_type_resolver::ReferenceTypeResolver;
use crate::types::{self, Type};
use std::rc::Rc;

pub struct TypeCompiler {
    reference_type_resolver: Rc<ReferenceTypeResolver>,
}

impl TypeCompiler {
    pub fn new(reference_type_resolver: impl Into<Rc<ReferenceTypeResolver>>) -> Self {
        Self {
            reference_type_resolver: reference_type_resolver.into(),
        }
    }

    pub fn compile(&self, type_: &Type) -> core::types::Type {
        match type_ {
            Type::Function(_) => self.compile_function(type_).into(),
            Type::Number(_) => core::types::Value::Number.into(),
            Type::Reference(_) => self.compile(&self.reference_type_resolver.resolve(type_)),
            Type::Variable(_) => unreachable!(),
        }
    }

    pub fn compile_function(&self, type_: &types::Type) -> core::types::Function {
        match type_ {
            Type::Function(function) => core::types::Function::new(
                function
                    .arguments()
                    .iter()
                    .map(|type_| self.compile(type_))
                    .collect::<Vec<_>>(),
                self.compile_value(function.last_result()),
            ),
            Type::Number(_) => unreachable!(),
            Type::Reference(_) => {
                self.compile_function(&self.reference_type_resolver.resolve(type_))
            }
            Type::Variable(_) => unreachable!(),
        }
    }

    pub fn compile_value(&self, type_: &Type) -> core::types::Value {
        match type_ {
            Type::Function(_) => unreachable!(),
            Type::Number(_) => core::types::Value::Number,
            Type::Reference(_) => self.compile_value(&self.reference_type_resolver.resolve(type_)),
            Type::Variable(_) => unreachable!(),
        }
    }
}
