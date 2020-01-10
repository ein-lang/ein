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
            Type::Unknown(_) | Type::Variable(_) => unreachable!(),
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
            Type::Unknown(_) | Type::Variable(_) => unreachable!(),
        }
    }

    pub fn compile_value(&self, type_: &Type) -> core::types::Value {
        match type_ {
            Type::Function(_) => unreachable!(),
            Type::Number(_) => core::types::Value::Number,
            Type::Reference(_) => self.compile_value(&self.reference_type_resolver.resolve(type_)),
            Type::Unknown(_) | Type::Variable(_) => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use crate::debug::SourceInformation;
    use crate::package::*;
    use crate::path::*;

    #[test]
    fn compile_number_type() {
        assert_eq!(
            TypeCompiler::new(ReferenceTypeResolver::new(&Module::dummy()))
                .compile(&types::Number::new(SourceInformation::dummy()).into()),
            core::types::Value::Number.into()
        );
    }

    #[test]
    fn compile_function_type() {
        assert_eq!(
            TypeCompiler::new(ReferenceTypeResolver::new(&Module::dummy())).compile(
                &types::Function::new(
                    types::Number::new(SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()
            ),
            core::types::Function::new(
                vec![core::types::Value::Number.into()],
                core::types::Value::Number
            )
            .into()
        );
    }

    #[test]
    fn compile_reference_type() {
        assert_eq!(
            TypeCompiler::new(ReferenceTypeResolver::new(&Module::new(
                ModulePath::new(Package::new("", ""), vec![]),
                Export::new(Default::default()),
                vec![],
                vec![TypeDefinition::new(
                    "Foo",
                    types::Number::new(SourceInformation::dummy()),
                )],
                vec![],
            )))
            .compile(&types::Reference::new("Foo", SourceInformation::dummy()).into()),
            core::types::Value::Number.into()
        );
    }
}
