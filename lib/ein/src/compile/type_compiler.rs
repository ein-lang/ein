use super::error::CompileError;
use super::reference_type_resolver::ReferenceTypeResolver;
use super::union_tag_calculator::UnionTagCalculator;
use crate::types::{self, Type};
use lazy_static::lazy_static;
use std::rc::Rc;

lazy_static! {
    static ref UNION_PADDING_ENTRY: (u64, ssf::types::Constructor) = (
        0,
        ssf::types::Constructor::unboxed(vec![ssf::types::Primitive::Integer64.into()]),
    );
}

pub struct TypeCompiler {
    record_names: Vec<Option<String>>,
    reference_type_resolver: Rc<ReferenceTypeResolver>,
    union_tag_calculator: Rc<UnionTagCalculator>,
}

impl TypeCompiler {
    pub fn new(
        reference_type_resolver: Rc<ReferenceTypeResolver>,
        union_tag_calculator: Rc<UnionTagCalculator>,
    ) -> Rc<Self> {
        Self {
            record_names: vec![],
            reference_type_resolver,
            union_tag_calculator,
        }
        .into()
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
            Type::Record(record) => Ok(self.compile_record_recursively(record)?.into()),
            Type::Reference(reference) => self.compile_reference(reference),
            Type::Union(union) => Ok(self.compile_union(union)?.into()),
            Type::Unknown(_) | Type::Variable(_) => unreachable!(),
        }
    }

    fn compile_reference(
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
        let other = self.push_record_name(Some(record.name().into()));
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

    fn compile_record_recursively(
        &self,
        record: &types::Record,
    ) -> Result<ssf::types::Value, CompileError> {
        Ok(
            if let Some(index) = self
                .record_names
                .iter()
                .rev()
                .position(|name| name.as_deref() == Some(record.name()))
            {
                ssf::types::Value::Index(index)
            } else {
                self.compile_record(record)?.into()
            },
        )
    }

    pub fn compile_union(
        &self,
        union: &types::Union,
    ) -> Result<ssf::types::Algebraic, CompileError> {
        let other = self.push_record_name(None);

        // Make sure that every union type has exactly 128 bits so that
        // we can omit union-to-union type coercion.
        Ok(ssf::types::Algebraic::with_tags(
            union
                .types()
                .iter()
                .map(|type_| {
                    let type_ = other.reference_type_resolver.resolve(type_)?;

                    Ok(match &type_ {
                        Type::Boolean(_)
                        | Type::Function(_)
                        | Type::None(_)
                        | Type::Number(_)
                        | Type::Record(_) => (
                            other.union_tag_calculator.calculate(&type_)?,
                            ssf::types::Constructor::unboxed(vec![other.compile(&type_)?]),
                        ),
                        Type::Reference(_)
                        | Type::Union(_)
                        | Type::Unknown(_)
                        | Type::Variable(_) => unreachable!(),
                    })
                })
                .chain(vec![Ok(UNION_PADDING_ENTRY.clone())])
                .collect::<Result<_, CompileError>>()?,
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

    fn push_record_name(&self, name: Option<String>) -> Self {
        Self {
            record_names: self
                .record_names
                .clone()
                .into_iter()
                .chain(vec![name])
                .collect(),
            reference_type_resolver: self.reference_type_resolver.clone(),
            union_tag_calculator: self.union_tag_calculator.clone(),
        }
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
        let reference_type_resolver = ReferenceTypeResolver::new(&Module::dummy());
        let union_tag_calculator = UnionTagCalculator::new(reference_type_resolver.clone());

        assert_eq!(
            TypeCompiler::new(reference_type_resolver, union_tag_calculator)
                .compile(&types::Number::new(SourceInformation::dummy()).into()),
            Ok(ssf::types::Primitive::Float64.into())
        );
    }

    #[test]
    fn compile_function_type() {
        let reference_type_resolver = ReferenceTypeResolver::new(&Module::dummy());
        let union_tag_calculator = UnionTagCalculator::new(reference_type_resolver.clone());

        assert_eq!(
            TypeCompiler::new(reference_type_resolver, union_tag_calculator).compile(
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
        let reference_type_resolver =
            ReferenceTypeResolver::new(&Module::from_definitions_and_type_definitions(
                vec![TypeDefinition::new(
                    "Foo",
                    types::Record::new(
                        "Bar",
                        vec![("foo".into(), reference_type.clone().into())]
                            .into_iter()
                            .collect(),
                        SourceInformation::dummy(),
                    ),
                )],
                vec![],
            ));
        let union_tag_calculator = UnionTagCalculator::new(reference_type_resolver.clone());

        assert_eq!(
            TypeCompiler::new(reference_type_resolver, union_tag_calculator)
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
        let reference_type_resolver =
            ReferenceTypeResolver::new(&Module::from_definitions_and_type_definitions(
                vec![
                    TypeDefinition::new(
                        "Foo",
                        types::Record::new(
                            "Foo",
                            vec![(
                                "foo".into(),
                                types::Reference::new("Bar", SourceInformation::dummy()).into(),
                            )]
                            .into_iter()
                            .collect(),
                            SourceInformation::dummy(),
                        ),
                    ),
                    TypeDefinition::new(
                        "Bar",
                        types::Record::new(
                            "Bar",
                            vec![("bar".into(), reference_type.clone().into())]
                                .into_iter()
                                .collect(),
                            SourceInformation::dummy(),
                        ),
                    ),
                ],
                vec![],
            ));
        let union_tag_calculator = UnionTagCalculator::new(reference_type_resolver.clone());

        assert_eq!(
            TypeCompiler::new(reference_type_resolver, union_tag_calculator)
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

    #[test]
    fn compile_nested_recursive_record_type_with_union_field() {
        let reference_type = types::Reference::new("Foo", SourceInformation::dummy());

        let reference_type_resolver =
            ReferenceTypeResolver::new(&Module::from_definitions_and_type_definitions(
                vec![
                    TypeDefinition::new(
                        "Foo",
                        types::Record::new(
                            "Foo",
                            vec![(
                                "foo".into(),
                                types::Reference::new("Bar", SourceInformation::dummy()).into(),
                            )]
                            .into_iter()
                            .collect(),
                            SourceInformation::dummy(),
                        ),
                    ),
                    TypeDefinition::new(
                        "Bar",
                        types::Record::new(
                            "Bar",
                            vec![(
                                "bar".into(),
                                types::Union::new(
                                    vec![
                                        reference_type.clone().into(),
                                        types::None::new(SourceInformation::dummy()).into(),
                                    ],
                                    SourceInformation::dummy(),
                                )
                                .into(),
                            )]
                            .into_iter()
                            .collect(),
                            SourceInformation::dummy(),
                        ),
                    ),
                ],
                vec![],
            ));

        assert_eq!(
            TypeCompiler::new(
                reference_type_resolver.clone(),
                UnionTagCalculator::new(reference_type_resolver.clone())
            )
            .compile(&reference_type.into()),
            Ok(
                ssf::types::Algebraic::new(vec![ssf::types::Constructor::new(
                    vec![
                        ssf::types::Algebraic::new(vec![ssf::types::Constructor::new(
                            vec![ssf::types::Algebraic::with_tags(
                                vec![
                                    UNION_PADDING_ENTRY.clone(),
                                    (
                                        5752548472714560345,
                                        ssf::types::Constructor::unboxed(vec![
                                            ssf::types::Algebraic::new(vec![
                                                ssf::types::Constructor::unboxed(vec![])
                                            ])
                                            .into()
                                        ])
                                    ),
                                    (
                                        461893210254723387,
                                        ssf::types::Constructor::new(
                                            vec![ssf::types::Value::Index(2).into()],
                                            false
                                        )
                                    )
                                ]
                                .into_iter()
                                .collect()
                            )
                            .into()],
                            true
                        )])
                        .into()
                    ],
                    true
                )])
                .into()
            )
        );

        assert_eq!(
            TypeCompiler::new(
                reference_type_resolver.clone(),
                UnionTagCalculator::new(reference_type_resolver.clone())
            )
            .compile(&types::Reference::new("Bar", SourceInformation::dummy()).into()),
            Ok(
                ssf::types::Algebraic::new(vec![ssf::types::Constructor::new(
                    vec![ssf::types::Algebraic::with_tags(
                        vec![
                            UNION_PADDING_ENTRY.clone(),
                            (
                                5752548472714560345,
                                ssf::types::Constructor::unboxed(vec![ssf::types::Algebraic::new(
                                    vec![ssf::types::Constructor::unboxed(vec![])]
                                )
                                .into()])
                            ),
                            (
                                461893210254723387,
                                ssf::types::Constructor::new(
                                    vec![ssf::types::Algebraic::new(vec![
                                        ssf::types::Constructor::new(
                                            vec![ssf::types::Value::Index(2).into()],
                                            true
                                        )
                                    ])
                                    .into()],
                                    false
                                )
                            )
                        ]
                        .into_iter()
                        .collect()
                    )
                    .into()],
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
            let reference_type_resolver =
                ReferenceTypeResolver::new(&Module::from_definitions_and_type_definitions(
                    vec![
                        TypeDefinition::new(
                            "Foo",
                            types::Record::new(
                                "Foo",
                                Default::default(),
                                SourceInformation::dummy(),
                            ),
                        ),
                        TypeDefinition::new(
                            "Bar",
                            types::Record::new(
                                "Bar",
                                Default::default(),
                                SourceInformation::dummy(),
                            ),
                        ),
                    ],
                    vec![],
                ));
            let union_tag_calculator = UnionTagCalculator::new(reference_type_resolver.clone());

            assert_eq!(
                TypeCompiler::new(reference_type_resolver, union_tag_calculator).compile(
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
                        UNION_PADDING_ENTRY.clone(),
                        (
                            461893210254723387,
                            ssf::types::Constructor::unboxed(vec![ssf::types::Algebraic::new(
                                vec![ssf::types::Constructor::unboxed(vec![])]
                            )
                            .into()])
                        ),
                        (
                            7277881248784541008,
                            ssf::types::Constructor::unboxed(vec![ssf::types::Algebraic::new(
                                vec![ssf::types::Constructor::unboxed(vec![])]
                            )
                            .into()])
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
            let reference_type_resolver = ReferenceTypeResolver::new(&Module::dummy());
            let union_tag_calculator = UnionTagCalculator::new(reference_type_resolver.clone());

            assert_eq!(
                TypeCompiler::new(reference_type_resolver, union_tag_calculator).compile(
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
                        UNION_PADDING_ENTRY.clone(),
                        (
                            4919337809186972848,
                            ssf::types::Constructor::unboxed(vec![ssf::types::Algebraic::new(
                                vec![
                                    ssf::types::Constructor::unboxed(vec![]),
                                    ssf::types::Constructor::unboxed(vec![])
                                ]
                            )
                            .into()])
                        ),
                        (
                            5752548472714560345,
                            ssf::types::Constructor::unboxed(vec![ssf::types::Algebraic::new(
                                vec![ssf::types::Constructor::unboxed(vec![])]
                            )
                            .into()])
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
            let reference_type_resolver = ReferenceTypeResolver::new(&Module::dummy());
            let union_tag_calculator = UnionTagCalculator::new(reference_type_resolver.clone());

            assert_eq!(
                TypeCompiler::new(reference_type_resolver, union_tag_calculator).compile(
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
                        UNION_PADDING_ENTRY.clone(),
                        (
                            17146441699440925146,
                            ssf::types::Constructor::unboxed(vec![
                                ssf::types::Primitive::Float64.into()
                            ])
                        ),
                        (
                            5752548472714560345,
                            ssf::types::Constructor::unboxed(vec![ssf::types::Algebraic::new(
                                vec![ssf::types::Constructor::unboxed(vec![])]
                            )
                            .into()])
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
