use super::error::CompileError;
use super::reference_type_resolver::ReferenceTypeResolver;
use crate::types::{self, Type};

#[derive(Debug)]
pub struct TypeCompiler<'a> {
    references: Vec<String>,
    reference_type_resolver: &'a ReferenceTypeResolver,
}

impl<'a> TypeCompiler<'a> {
    pub fn new(reference_type_resolver: &'a ReferenceTypeResolver) -> Self {
        Self {
            references: vec![],
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
        if let Some(index) = self
            .references
            .iter()
            .rev()
            .position(|name| name == reference.name())
        {
            Ok(ssf::types::Value::Index(index).into())
        } else {
            self.compile(&self.reference_type_resolver.resolve_reference(reference)?)
        }
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
        let other = self.push_reference(record.name());

        Ok(ssf::types::Algebraic::new(vec![
            ssf::types::Constructor::boxed(
                record
                    .elements()
                    .iter()
                    .map(|(_, type_)| other.compile(type_))
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

    fn push_reference(&self, reference: &str) -> Self {
        Self {
            references: self
                .references
                .clone()
                .into_iter()
                .chain(vec![reference.into()])
                .collect(),
            reference_type_resolver: self.reference_type_resolver,
        }
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

    #[test]
    fn compile_recursive_record_type() {
        let reference_type = types::Reference::new("Foo", SourceInformation::dummy());

        assert_eq!(
            TypeCompiler::new(&ReferenceTypeResolver::new(
                &Module::from_definitions_and_type_definitions(
                    vec![TypeDefinition::new(
                        "Foo",
                        types::Record::new(
                            "Foo",
                            vec![("foo".into(), reference_type.clone().into())]
                                .into_iter()
                                .collect(),
                            SourceInformation::dummy()
                        )
                    )],
                    vec![]
                )
            ))
            .compile(&reference_type.into()),
            Ok(
                ssf::types::Algebraic::new(vec![ssf::types::Constructor::new(
                    vec![ssf::types::Value::Index(0).into()],
                    true
                )])
                .into()
            )
        );
    }

    #[test]
    fn compile_nested_recursive_record_type() {
        let reference_type = types::Reference::new("Foo", SourceInformation::dummy());

        assert_eq!(
            TypeCompiler::new(&ReferenceTypeResolver::new(
                &Module::from_definitions_and_type_definitions(
                    vec![
                        TypeDefinition::new(
                            "Foo",
                            types::Record::new(
                                "Foo",
                                vec![(
                                    "foo".into(),
                                    types::Reference::new("Bar", SourceInformation::dummy()).into()
                                )]
                                .into_iter()
                                .collect(),
                                SourceInformation::dummy()
                            )
                        ),
                        TypeDefinition::new(
                            "Bar",
                            types::Record::new(
                                "Bar",
                                vec![("bar".into(), reference_type.clone().into())]
                                    .into_iter()
                                    .collect(),
                                SourceInformation::dummy()
                            )
                        )
                    ],
                    vec![]
                )
            ))
            .compile(&reference_type.into()),
            Ok(
                ssf::types::Algebraic::new(vec![ssf::types::Constructor::new(
                    vec![
                        ssf::types::Algebraic::new(vec![ssf::types::Constructor::new(
                            vec![ssf::types::Value::Index(1).into()],
                            true
                        )])
                        .into()
                    ],
                    true
                )])
                .into()
            )
        );
    }
}
