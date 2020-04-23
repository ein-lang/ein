use super::error::CompileError;
use super::reference_type_resolver::ReferenceTypeResolver;
use crate::types::{self, Type};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

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
            Type::Number(_) => Ok(self.compile_number().into()),
            Type::Record(record) => Ok(self.compile_record(record)?.into()),
            Type::Reference(reference) => self.compile_reference(reference),
            Type::Union(union) => Ok(self.compile_union(union)?.into()),
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
        let elements = record
            .elements()
            .iter()
            .map(|(_, type_)| other.compile(type_))
            .collect::<Result<Vec<_>, _>>()?;
        let is_boxed = !elements.is_empty();

        Ok(ssf::types::Algebraic::new(vec![
            ssf::types::Constructor::new(elements, is_boxed),
        ]))
    }

    pub fn compile_union(
        &self,
        union: &types::Union,
    ) -> Result<ssf::types::Algebraic, CompileError> {
        Ok(ssf::types::Algebraic::with_tags(
            union
                .types()
                .iter()
                .map(|type_| {
                    let type_ = self.reference_type_resolver.resolve(type_)?;

                    Ok(match type_ {
                        Type::Boolean(_) => vec![
                            (
                                self.calculate_constructor_tag("false"),
                                ssf::types::Constructor::unboxed(vec![]),
                            ),
                            (
                                self.calculate_constructor_tag("true"),
                                ssf::types::Constructor::unboxed(vec![]),
                            ),
                        ],
                        Type::Function(_) => vec![(
                            self.calculate_constructor_tag(
                                &self.calculate_function_constructor_id(&type_)?,
                            ),
                            ssf::types::Constructor::unboxed(vec![self.compile(&type_)?]),
                        )],
                        Type::None(_) => vec![(
                            self.calculate_constructor_tag("none"),
                            ssf::types::Constructor::unboxed(vec![]),
                        )],
                        Type::Number(_) => vec![(
                            self.calculate_constructor_tag("number"),
                            ssf::types::Constructor::unboxed(vec![self.compile_number().into()]),
                        )],
                        Type::Record(record) => vec![(
                            self.calculate_constructor_tag(record.name()), // TODO
                            self.compile_record(&record)?.unfold().constructors()[&0].clone(),
                        )],
                        Type::Reference(_)
                        | Type::Union(_)
                        | Type::Unknown(_)
                        | Type::Variable(_) => unreachable!(),
                    })
                })
                .collect::<Result<Vec<_>, CompileError>>()?
                .into_iter()
                .flatten()
                .collect(),
        ))
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

    fn compile_number(&self) -> ssf::types::Primitive {
        ssf::types::Primitive::Float64
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

    fn calculate_constructor_tag(&self, constructor_id: &str) -> u64 {
        let mut hasher = DefaultHasher::new();

        constructor_id.hash(&mut hasher);

        hasher.finish()
    }

    fn calculate_function_constructor_id(&self, type_: &Type) -> Result<String, CompileError> {
        Ok(match self.reference_type_resolver.resolve(type_)? {
            Type::Boolean(_) => "Boolean".into(),
            Type::Function(function) => format!(
                "({}->{})",
                self.calculate_function_constructor_id(function.argument())?,
                self.calculate_function_constructor_id(function.result())?
            ),
            Type::None(_) => "None".into(),
            Type::Number(_) => "Number".into(),
            Type::Record(record) => record.name().into(),
            Type::Reference(_) | Type::Union(_) | Type::Unknown(_) | Type::Variable(_) => {
                unreachable!()
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use crate::debug::SourceInformation;
    use pretty_assertions::assert_eq;

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

    mod union {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn compile_union_type_of_records() {
            assert_eq!(
                TypeCompiler::new(&ReferenceTypeResolver::new(
                    &Module::from_definitions_and_type_definitions(
                        vec![
                            TypeDefinition::new(
                                "Foo",
                                types::Record::new(
                                    "Foo",
                                    Default::default(),
                                    SourceInformation::dummy()
                                )
                            ),
                            TypeDefinition::new(
                                "Bar",
                                types::Record::new(
                                    "Bar",
                                    Default::default(),
                                    SourceInformation::dummy()
                                )
                            )
                        ],
                        vec![]
                    )
                ))
                .compile(
                    &types::Union::new(
                        vec![
                            types::Reference::new("Foo", SourceInformation::dummy()).into(),
                            types::Reference::new("Bar", SourceInformation::dummy()).into()
                        ],
                        SourceInformation::dummy()
                    )
                    .into(),
                ),
                Ok(ssf::types::Algebraic::with_tags(
                    vec![
                        (461893210254723387, ssf::types::Constructor::unboxed(vec![])),
                        (
                            7277881248784541008,
                            ssf::types::Constructor::unboxed(vec![])
                        )
                    ]
                    .into_iter()
                    .collect()
                )
                .into())
            );
        }

        #[test]
        fn compile_union_type_including_boolean() {
            assert_eq!(
                TypeCompiler::new(&ReferenceTypeResolver::new(&Module::dummy())).compile(
                    &types::Union::new(
                        vec![
                            types::Boolean::new(SourceInformation::dummy()).into(),
                            types::None::new(SourceInformation::dummy()).into()
                        ],
                        SourceInformation::dummy()
                    )
                    .into(),
                ),
                Ok(ssf::types::Algebraic::with_tags(
                    vec![
                        (
                            2326242343701258586,
                            ssf::types::Constructor::unboxed(vec![])
                        ),
                        (
                            8985926696363166359,
                            ssf::types::Constructor::unboxed(vec![])
                        ),
                        (
                            15278957102451735707,
                            ssf::types::Constructor::unboxed(vec![])
                        )
                    ]
                    .into_iter()
                    .collect()
                )
                .into())
            );
        }

        #[test]
        fn compile_union_type_including_number() {
            assert_eq!(
                TypeCompiler::new(&ReferenceTypeResolver::new(&Module::dummy())).compile(
                    &types::Union::new(
                        vec![
                            types::Number::new(SourceInformation::dummy()).into(),
                            types::None::new(SourceInformation::dummy()).into()
                        ],
                        SourceInformation::dummy()
                    )
                    .into(),
                ),
                Ok(ssf::types::Algebraic::with_tags(
                    vec![
                        (
                            732683288442843147,
                            ssf::types::Constructor::unboxed(vec![
                                ssf::types::Primitive::Float64.into()
                            ])
                        ),
                        (
                            15278957102451735707,
                            ssf::types::Constructor::unboxed(vec![])
                        )
                    ]
                    .into_iter()
                    .collect()
                )
                .into())
            );
        }
    }
}
