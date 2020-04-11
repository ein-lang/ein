use super::error::CompileError;
use super::reference_type_resolver::ReferenceTypeResolver;
use crate::types::{self, Type};
use std::collections::HashMap;

#[derive(Debug)]
pub struct TypeCompiler<'a> {
    reference_indices: HashMap<String, usize>,
    reference_type_resolver: &'a ReferenceTypeResolver,
}

impl<'a> TypeCompiler<'a> {
    pub fn new(reference_type_resolver: &'a ReferenceTypeResolver) -> Self {
        Self {
            reference_indices: HashMap::new(),
            reference_type_resolver,
        }
    }

    pub fn compile(&self, type_: &Type) -> Result<ssf::types::Type, CompileError> {
        match type_ {
            Type::Boolean(_) => Ok(self.compile_boolean().into()),
            Type::Function(function) => Ok(ssf::types::Function::new(
                function
                    .arguments()
                    .iter()
                    .map(|type_| self.compile(type_))
                    .collect::<Result<_, _>>()?,
                self.compile_value(function.last_result())?,
            )
            .into()),
            Type::None(_) => Ok(self.compile_none().into()),
            Type::Number(_) => Ok(ssf::types::Primitive::Float64.into()),
            Type::Record(record) => Ok(self.compile_record(record)?.into()),
            Type::Reference(reference) => self.compile_reference(reference),
            Type::Unknown(_) | Type::Variable(_) => unreachable!(),
        }
    }

    pub fn compile_reference(
        &self,
        reference: &types::Reference,
    ) -> Result<ssf::types::Type, CompileError> {
        self.compile(&self.reference_type_resolver.resolve_reference(reference)?)
    }

    pub fn compile_function(
        &self,
        type_: &types::Type,
    ) -> Result<ssf::types::Function, CompileError> {
        Ok(self.compile(type_)?.into_function().unwrap())
    }

    pub fn compile_record(
        &self,
        record: &types::Record,
    ) -> Result<ssf::types::Algebraic, CompileError> {
        Ok(ssf::types::Algebraic::new(vec![
            ssf::types::Constructor::boxed(
                record
                    .elements()
                    .iter()
                    .map(|(_, type_)| self.compile(type_))
                    .collect::<Result<_, _>>()?,
            ),
        ]))
    }

    pub fn compile_value(&self, type_: &Type) -> Result<ssf::types::Value, CompileError> {
        Ok(self.compile(type_)?.into_value().unwrap())
    }

    pub fn compile_boolean(&self) -> ssf::types::Algebraic {
        ssf::types::Algebraic::new(vec![
            ssf::types::Constructor::unboxed(vec![]),
            ssf::types::Constructor::unboxed(vec![]),
        ])
    }

    pub fn compile_none(&self) -> ssf::types::Algebraic {
        ssf::types::Algebraic::new(vec![ssf::types::Constructor::unboxed(vec![])])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use crate::debug::SourceInformation;

    #[test]
    fn compile_number_type() {
        assert_eq!(
            TypeCompiler::new(&ReferenceTypeResolver::new(&Module::dummy()))
                .compile(&types::Number::new(SourceInformation::dummy()).into()),
            Ok(ssf::types::Primitive::Float64.into())
        );
    }

    #[test]
    fn compile_function_type() {
        assert_eq!(
            TypeCompiler::new(&ReferenceTypeResolver::new(&Module::dummy())).compile(
                &types::Function::new(
                    types::Number::new(SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()
            ),
            Ok(ssf::types::Function::new(
                vec![ssf::types::Primitive::Float64.into()],
                ssf::types::Primitive::Float64
            )
            .into())
        );
    }
}
