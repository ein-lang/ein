mod boolean_operation_desugarer;
mod equal_operation_desugarer;
mod function_type_argument_desugarer;
mod not_equal_operation_desugarer;
mod partial_application_desugarer;
mod record_function_desugarer;
mod record_update_desugarer;
mod type_coercion_desugarer;
mod typed_meta_desugarer;

use super::error::CompileError;
use super::expression_type_extractor::ExpressionTypeExtractor;
use super::reference_type_resolver::ReferenceTypeResolver;
use super::type_equality_checker::TypeEqualityChecker;
use super::union_type_simplifier::UnionTypeSimplifier;
use crate::ast::*;
use boolean_operation_desugarer::BooleanOperationDesugarer;
use equal_operation_desugarer::EqualOperationDesugarer;
use function_type_argument_desugarer::FunctionTypeArgumentDesugarer;
use not_equal_operation_desugarer::NotEqualOperationDesugarer;
use partial_application_desugarer::PartialApplicationDesugarer;
use record_function_desugarer::RecordFunctionDesugarer;
use record_update_desugarer::RecordUpdateDesugarer;
use type_coercion_desugarer::TypeCoercionDesugarer;
use typed_meta_desugarer::TypedMetaDesugarer;

pub fn desugar_before_name_qualification(module: &Module) -> Result<Module, CompileError> {
    Ok(RecordFunctionDesugarer::new().desugar(module))
}

pub fn desugar_without_types(module: &Module) -> Result<Module, CompileError> {
    RecordUpdateDesugarer::new().desugar(module)
}

pub fn desugar_with_types(module: &Module) -> Result<Module, CompileError> {
    let reference_type_resolver = ReferenceTypeResolver::new(module);
    let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());
    let union_type_simplifier = UnionTypeSimplifier::new(reference_type_resolver.clone());
    let expression_type_extractor = ExpressionTypeExtractor::new(
        reference_type_resolver.clone(),
        union_type_simplifier.clone(),
    );

    let module = BooleanOperationDesugarer::new().desugar(&module)?;

    let module = NotEqualOperationDesugarer::new().desugar(&module)?;
    let module = EqualOperationDesugarer::new(
        reference_type_resolver.clone(),
        type_equality_checker.clone(),
    )
    .desugar(&module)?;

    let module = TypedMetaDesugarer::new(FunctionTypeArgumentDesugarer::new(
        reference_type_resolver.clone(),
        type_equality_checker.clone(),
        expression_type_extractor.clone(),
    ))
    .desugar(&module)?;

    let module = PartialApplicationDesugarer::new().desugar(&module)?;

    TypedMetaDesugarer::new(TypeCoercionDesugarer::new(
        reference_type_resolver,
        type_equality_checker,
        expression_type_extractor,
        union_type_simplifier,
    ))
    .desugar(&module)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug::SourceInformation;
    use crate::types;
    use insta::assert_debug_snapshot;
    use pretty_assertions::assert_eq;

    mod type_coercion {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn desugar_function_definition() {
            let union_type = types::Union::new(
                vec![
                    types::Number::new(SourceInformation::dummy()).into(),
                    types::None::new(SourceInformation::dummy()).into(),
                ],
                SourceInformation::dummy(),
            );

            let create_module = |body: Expression| {
                Module::from_definitions(vec![
                    FunctionDefinition::new(
                        "f",
                        vec!["x".into()],
                        Number::new(42.0, SourceInformation::dummy()),
                        types::Function::new(
                            union_type.clone(),
                            types::Number::new(SourceInformation::dummy()),
                            SourceInformation::dummy(),
                        ),
                        SourceInformation::dummy(),
                    )
                    .into(),
                    FunctionDefinition::new(
                        "g",
                        vec!["x".into()],
                        body,
                        types::Function::new(
                            types::Number::new(SourceInformation::dummy()),
                            union_type.clone(),
                            SourceInformation::dummy(),
                        ),
                        SourceInformation::dummy(),
                    )
                    .into(),
                ])
            };

            assert_eq!(
                desugar_with_types(&create_module(
                    Application::new(
                        Variable::new("f", SourceInformation::dummy()),
                        Number::new(42.0, SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    )
                    .into()
                )),
                Ok(create_module(
                    TypeCoercion::new(
                        Application::new(
                            Variable::new("f", SourceInformation::dummy()),
                            TypeCoercion::new(
                                Number::new(42.0, SourceInformation::dummy()),
                                types::Number::new(SourceInformation::dummy()),
                                union_type.clone(),
                                SourceInformation::dummy(),
                            ),
                            SourceInformation::dummy(),
                        ),
                        types::Number::new(SourceInformation::dummy()),
                        union_type.clone(),
                        SourceInformation::dummy(),
                    )
                    .into()
                ))
            );
        }

        #[test]
        fn desugar_value_definition() {
            let union_type = types::Union::new(
                vec![
                    types::Number::new(SourceInformation::dummy()).into(),
                    types::None::new(SourceInformation::dummy()).into(),
                ],
                SourceInformation::dummy(),
            );

            let create_module = |expression: Expression| {
                Module::from_definitions(vec![ValueDefinition::new(
                    "x",
                    expression,
                    union_type.clone(),
                    SourceInformation::dummy(),
                )
                .into()])
            };

            assert_eq!(
                desugar_with_types(&create_module(
                    Number::new(42.0, SourceInformation::dummy()).into()
                )),
                Ok(create_module(
                    TypeCoercion::new(
                        Number::new(42.0, SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        union_type.clone(),
                        SourceInformation::dummy(),
                    )
                    .into()
                ))
            );
        }

        #[test]
        fn desugar_application() {
            let union_type = types::Union::new(
                vec![
                    types::Number::new(SourceInformation::dummy()).into(),
                    types::None::new(SourceInformation::dummy()).into(),
                ],
                SourceInformation::dummy(),
            );

            let create_module = |argument: Expression| {
                Module::from_definitions(vec![
                    FunctionDefinition::new(
                        "f",
                        vec!["x".into()],
                        Number::new(42.0, SourceInformation::dummy()),
                        types::Function::new(
                            union_type.clone(),
                            types::Number::new(SourceInformation::dummy()),
                            SourceInformation::dummy(),
                        ),
                        SourceInformation::dummy(),
                    )
                    .into(),
                    ValueDefinition::new(
                        "x",
                        Application::new(
                            Variable::new("f", SourceInformation::dummy()),
                            argument,
                            SourceInformation::dummy(),
                        ),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    )
                    .into(),
                ])
            };

            assert_eq!(
                desugar_with_types(&create_module(
                    Number::new(42.0, SourceInformation::dummy()).into()
                )),
                Ok(create_module(
                    TypeCoercion::new(
                        Number::new(42.0, SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        union_type.clone(),
                        SourceInformation::dummy(),
                    )
                    .into()
                ))
            );
        }

        #[test]
        fn desugar_let_value_expression() {
            let union_type = types::Union::new(
                vec![
                    types::Number::new(SourceInformation::dummy()).into(),
                    types::None::new(SourceInformation::dummy()).into(),
                ],
                SourceInformation::dummy(),
            );

            let create_module = |expression: Expression| {
                Module::from_definitions(vec![ValueDefinition::new(
                    "x",
                    Let::new(
                        vec![ValueDefinition::new(
                            "y",
                            expression,
                            union_type.clone(),
                            SourceInformation::dummy(),
                        )
                        .into()],
                        Number::new(42.0, SourceInformation::dummy()),
                    ),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()])
            };

            assert_eq!(
                desugar_with_types(&create_module(
                    Number::new(42.0, SourceInformation::dummy()).into()
                )),
                Ok(create_module(
                    TypeCoercion::new(
                        Number::new(42.0, SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        union_type.clone(),
                        SourceInformation::dummy(),
                    )
                    .into()
                ))
            );
        }

        #[test]
        fn desugar_let_function_expression() {
            let union_type = types::Union::new(
                vec![
                    types::Number::new(SourceInformation::dummy()).into(),
                    types::None::new(SourceInformation::dummy()).into(),
                ],
                SourceInformation::dummy(),
            );

            let create_module = |body: Expression| {
                Module::from_definitions(vec![
                    FunctionDefinition::new(
                        "f",
                        vec!["x".into()],
                        Number::new(42.0, SourceInformation::dummy()),
                        types::Function::new(
                            union_type.clone(),
                            types::Number::new(SourceInformation::dummy()),
                            SourceInformation::dummy(),
                        ),
                        SourceInformation::dummy(),
                    )
                    .into(),
                    ValueDefinition::new(
                        "x",
                        Let::new(
                            vec![FunctionDefinition::new(
                                "g",
                                vec!["x".into()],
                                body,
                                types::Function::new(
                                    types::Number::new(SourceInformation::dummy()),
                                    union_type.clone(),
                                    SourceInformation::dummy(),
                                ),
                                SourceInformation::dummy(),
                            )
                            .into()],
                            Number::new(42.0, SourceInformation::dummy()),
                        ),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    )
                    .into(),
                ])
            };

            assert_eq!(
                desugar_with_types(&create_module(
                    Application::new(
                        Variable::new("f", SourceInformation::dummy()),
                        Number::new(42.0, SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    )
                    .into()
                )),
                Ok(create_module(
                    TypeCoercion::new(
                        Application::new(
                            Variable::new("f", SourceInformation::dummy()),
                            TypeCoercion::new(
                                Number::new(42.0, SourceInformation::dummy()),
                                types::Number::new(SourceInformation::dummy()),
                                union_type.clone(),
                                SourceInformation::dummy(),
                            ),
                            SourceInformation::dummy(),
                        ),
                        types::Number::new(SourceInformation::dummy()),
                        union_type.clone(),
                        SourceInformation::dummy(),
                    )
                    .into()
                ))
            );
        }

        #[test]
        fn desugar_record_construction() {
            let union_type = types::Union::new(
                vec![
                    types::Number::new(SourceInformation::dummy()).into(),
                    types::None::new(SourceInformation::dummy()).into(),
                ],
                SourceInformation::dummy(),
            );
            let reference_type = types::Reference::new("Foo", SourceInformation::dummy());

            assert_debug_snapshot!(desugar_with_types(
                &Module::from_definitions_and_type_definitions(
                    vec![TypeDefinition::new(
                        "Foo",
                        types::Record::new(
                            "Foo",
                            vec![("foo".into(), union_type.clone().into())]
                                .into_iter()
                                .collect(),
                            SourceInformation::dummy(),
                        )
                    )],
                    vec![ValueDefinition::new(
                        "x",
                        RecordConstruction::new(
                            reference_type.clone(),
                            vec![(
                                "foo".into(),
                                Number::new(42.0, SourceInformation::dummy()).into()
                            )]
                            .into_iter()
                            .collect(),
                            SourceInformation::dummy(),
                        ),
                        reference_type.clone(),
                        SourceInformation::dummy(),
                    )
                    .into()],
                )
            ));
        }

        #[test]
        fn desugar_union() {
            let lower_union_type = types::Union::new(
                vec![
                    types::Number::new(SourceInformation::dummy()).into(),
                    types::None::new(SourceInformation::dummy()).into(),
                ],
                SourceInformation::dummy(),
            );
            let upper_union_type = types::Union::new(
                vec![
                    types::Boolean::new(SourceInformation::dummy()).into(),
                    types::Number::new(SourceInformation::dummy()).into(),
                    types::None::new(SourceInformation::dummy()).into(),
                ],
                SourceInformation::dummy(),
            );

            let create_module = |expression1: Expression, expression2: Expression| {
                Module::from_definitions(vec![
                    ValueDefinition::new(
                        "x",
                        expression1,
                        lower_union_type.clone(),
                        SourceInformation::dummy(),
                    )
                    .into(),
                    ValueDefinition::new(
                        "y",
                        expression2,
                        upper_union_type.clone(),
                        SourceInformation::dummy(),
                    )
                    .into(),
                ])
            };

            assert_eq!(
                desugar_with_types(&create_module(
                    Number::new(42.0, SourceInformation::dummy()).into(),
                    Variable::new("x", SourceInformation::dummy()).into()
                )),
                Ok(create_module(
                    TypeCoercion::new(
                        Number::new(42.0, SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        lower_union_type.clone(),
                        SourceInformation::dummy(),
                    )
                    .into(),
                    TypeCoercion::new(
                        Variable::new("x", SourceInformation::dummy()),
                        lower_union_type.clone(),
                        upper_union_type.clone(),
                        SourceInformation::dummy(),
                    )
                    .into()
                ))
            );
        }

        #[test]
        fn desugar_function() {
            let lower_type = types::None::new(SourceInformation::dummy());
            let upper_type = types::Union::new(
                vec![
                    types::Boolean::new(SourceInformation::dummy()).into(),
                    types::None::new(SourceInformation::dummy()).into(),
                ],
                SourceInformation::dummy(),
            );

            let create_module = |definition: Definition| {
                Module::from_definitions(vec![
                    FunctionDefinition::new(
                        "f",
                        vec!["x".into()],
                        None::new(SourceInformation::dummy()),
                        types::Function::new(
                            upper_type.clone(),
                            lower_type.clone(),
                            SourceInformation::dummy(),
                        ),
                        SourceInformation::dummy(),
                    )
                    .into(),
                    ValueDefinition::new(
                        "x",
                        Let::new(vec![definition], None::new(SourceInformation::dummy())),
                        types::None::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    )
                    .into(),
                ])
            };

            assert_eq!(
                desugar_with_types(&create_module(
                    ValueDefinition::new(
                        "g",
                        Variable::new("f", SourceInformation::dummy()),
                        types::Function::new(
                            lower_type.clone(),
                            upper_type.clone(),
                            SourceInformation::dummy(),
                        ),
                        SourceInformation::dummy(),
                    )
                    .into()
                )),
                Ok(create_module(
                    FunctionDefinition::new(
                        "g",
                        vec!["pa_argument_0".into()],
                        TypeCoercion::new(
                            Application::new(
                                Variable::new("f", SourceInformation::dummy()),
                                TypeCoercion::new(
                                    Variable::new("pa_argument_0", SourceInformation::dummy()),
                                    lower_type.clone(),
                                    upper_type.clone(),
                                    SourceInformation::dummy(),
                                ),
                                SourceInformation::dummy()
                            ),
                            lower_type.clone(),
                            upper_type.clone(),
                            SourceInformation::dummy(),
                        ),
                        types::Function::new(
                            lower_type.clone(),
                            upper_type.clone(),
                            SourceInformation::dummy(),
                        ),
                        SourceInformation::dummy(),
                    )
                    .into()
                ))
            );
        }

        #[test]
        fn desugar_function_as_argument() {
            let lower_type = types::None::new(SourceInformation::dummy());
            let upper_type = types::Union::new(
                vec![
                    types::Boolean::new(SourceInformation::dummy()).into(),
                    types::None::new(SourceInformation::dummy()).into(),
                ],
                SourceInformation::dummy(),
            );

            let create_module = |expression: Expression| {
                Module::from_definitions(vec![
                    FunctionDefinition::new(
                        "f",
                        vec!["x".into()],
                        None::new(SourceInformation::dummy()),
                        types::Function::new(
                            upper_type.clone(),
                            lower_type.clone(),
                            SourceInformation::dummy(),
                        ),
                        SourceInformation::dummy(),
                    )
                    .into(),
                    FunctionDefinition::new(
                        "g",
                        vec!["x".into()],
                        None::new(SourceInformation::dummy()),
                        types::Function::new(
                            types::Function::new(
                                lower_type.clone(),
                                upper_type.clone(),
                                SourceInformation::dummy(),
                            ),
                            lower_type.clone(),
                            SourceInformation::dummy(),
                        ),
                        SourceInformation::dummy(),
                    )
                    .into(),
                    ValueDefinition::new(
                        "x",
                        Application::new(
                            Variable::new("g", SourceInformation::dummy()),
                            expression,
                            SourceInformation::dummy(),
                        ),
                        lower_type.clone(),
                        SourceInformation::dummy(),
                    )
                    .into(),
                ])
            };

            assert_eq!(
                desugar_with_types(&create_module(
                    Variable::new("f", SourceInformation::dummy()).into()
                )),
                Ok(create_module(
                    Let::new(
                        vec![FunctionDefinition::new(
                            "fta_function_0",
                            vec!["pa_argument_0".into()],
                            TypeCoercion::new(
                                Application::new(
                                    Variable::new("f", SourceInformation::dummy()),
                                    TypeCoercion::new(
                                        Variable::new("pa_argument_0", SourceInformation::dummy()),
                                        lower_type.clone(),
                                        upper_type.clone(),
                                        SourceInformation::dummy(),
                                    ),
                                    SourceInformation::dummy()
                                ),
                                lower_type.clone(),
                                upper_type.clone(),
                                SourceInformation::dummy(),
                            ),
                            types::Function::new(
                                lower_type.clone(),
                                upper_type.clone(),
                                SourceInformation::dummy(),
                            ),
                            SourceInformation::dummy(),
                        )
                        .into()],
                        Variable::new("fta_function_0", SourceInformation::dummy())
                    )
                    .into()
                ))
            );
        }
    }

    #[test]
    fn desugar_case_expression() {
        let argument_union_type = types::Union::new(
            vec![
                types::Boolean::new(SourceInformation::dummy()).into(),
                types::None::new(SourceInformation::dummy()).into(),
            ],
            SourceInformation::dummy(),
        );
        let result_union_type = types::Union::new(
            vec![
                types::Number::new(SourceInformation::dummy()).into(),
                types::None::new(SourceInformation::dummy()).into(),
            ],
            SourceInformation::dummy(),
        );

        let create_module = |expression: Expression| {
            Module::from_definitions(vec![ValueDefinition::new(
                "x",
                expression,
                result_union_type.clone(),
                SourceInformation::dummy(),
            )
            .into()])
        };

        assert_eq!(
            desugar_with_types(&create_module(
                Case::with_type(
                    argument_union_type.clone(),
                    "foo",
                    Boolean::new(false, SourceInformation::dummy()),
                    vec![
                        Alternative::new(
                            types::Boolean::new(SourceInformation::dummy()),
                            Number::new(42.0, SourceInformation::dummy())
                        ),
                        Alternative::new(
                            types::None::new(SourceInformation::dummy()),
                            None::new(SourceInformation::dummy())
                        )
                    ],
                    SourceInformation::dummy()
                )
                .into()
            )),
            Ok(create_module(
                Case::with_type(
                    argument_union_type.clone(),
                    "foo",
                    Boolean::new(false, SourceInformation::dummy()),
                    vec![
                        Alternative::new(
                            types::Boolean::new(SourceInformation::dummy()),
                            TypeCoercion::new(
                                Number::new(42.0, SourceInformation::dummy()),
                                types::Number::new(SourceInformation::dummy()),
                                result_union_type.clone(),
                                SourceInformation::dummy(),
                            )
                        ),
                        Alternative::new(
                            types::None::new(SourceInformation::dummy()),
                            TypeCoercion::new(
                                None::new(SourceInformation::dummy()),
                                types::None::new(SourceInformation::dummy()),
                                result_union_type.clone(),
                                SourceInformation::dummy(),
                            )
                        )
                    ],
                    SourceInformation::dummy()
                )
                .into()
            ))
        );
    }

    #[test]
    fn desugar_union_equal_operation() {
        let union_type = types::Union::new(
            vec![
                types::Number::new(SourceInformation::dummy()).into(),
                types::None::new(SourceInformation::dummy()).into(),
            ],
            SourceInformation::dummy(),
        );

        let create_module = |expression: Expression| {
            Module::from_definitions(vec![ValueDefinition::new(
                "x",
                expression,
                types::Boolean::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()])
        };

        assert_eq!(
            desugar_with_types(&create_module(
                Operation::with_type(
                    union_type.clone(),
                    Operator::Equal,
                    None::new(SourceInformation::dummy()),
                    None::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()
            )),
            Ok(create_module(
                Case::with_type(
                    union_type.clone(),
                    "equal_operation_argument_0",
                    None::new(SourceInformation::dummy()),
                    vec![
                        Alternative::new(
                            types::None::new(SourceInformation::dummy()),
                            Case::with_type(
                                union_type.clone(),
                                "equal_operation_argument_1",
                                None::new(SourceInformation::dummy()),
                                vec![
                                    Alternative::new(
                                        types::None::new(SourceInformation::dummy()),
                                        Boolean::new(true, SourceInformation::dummy())
                                    ),
                                    Alternative::new(
                                        types::Number::new(SourceInformation::dummy()),
                                        Boolean::new(false, SourceInformation::dummy())
                                    ),
                                ],
                                SourceInformation::dummy(),
                            )
                        ),
                        Alternative::new(
                            types::Number::new(SourceInformation::dummy()),
                            Case::with_type(
                                union_type.clone(),
                                "equal_operation_argument_1",
                                None::new(SourceInformation::dummy()),
                                vec![
                                    Alternative::new(
                                        types::None::new(SourceInformation::dummy()),
                                        Boolean::new(false, SourceInformation::dummy())
                                    ),
                                    Alternative::new(
                                        types::Number::new(SourceInformation::dummy()),
                                        Operation::with_type(
                                            types::Number::new(SourceInformation::dummy()),
                                            Operator::Equal,
                                            Variable::new(
                                                "equal_operation_argument_0",
                                                SourceInformation::dummy()
                                            ),
                                            Variable::new(
                                                "equal_operation_argument_1",
                                                SourceInformation::dummy()
                                            ),
                                            SourceInformation::dummy()
                                        )
                                    ),
                                ],
                                SourceInformation::dummy(),
                            )
                        ),
                    ],
                    SourceInformation::dummy(),
                )
                .into(),
            ))
        );
    }

    #[test]
    fn desugar_not_equal_operation() {
        let create_module = |expression: Expression| {
            Module::from_definitions(vec![ValueDefinition::new(
                "x",
                expression,
                types::Boolean::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()])
        };

        assert_eq!(
            desugar_with_types(&create_module(
                Operation::with_type(
                    types::Number::new(SourceInformation::dummy()),
                    Operator::NotEqual,
                    Number::new(42.0, SourceInformation::dummy()),
                    Number::new(42.0, SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()
            )),
            Ok(create_module(
                If::new(
                    Operation::with_type(
                        types::Number::new(SourceInformation::dummy()),
                        Operator::Equal,
                        Number::new(42.0, SourceInformation::dummy()),
                        Number::new(42.0, SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    Boolean::new(false, SourceInformation::dummy()),
                    Boolean::new(true, SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into(),
            ))
        );
    }

    #[test]
    fn desugar_and_operation() {
        let create_module = |expression: Expression| {
            Module::from_definitions(vec![ValueDefinition::new(
                "x",
                expression,
                types::Boolean::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()])
        };

        assert_eq!(
            desugar_with_types(&create_module(
                Operation::with_type(
                    types::Boolean::new(SourceInformation::dummy()),
                    Operator::And,
                    Boolean::new(true, SourceInformation::dummy()),
                    Boolean::new(true, SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()
            )),
            Ok(create_module(
                If::new(
                    Boolean::new(true, SourceInformation::dummy()),
                    Boolean::new(true, SourceInformation::dummy()),
                    Boolean::new(false, SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into(),
            ))
        );
    }

    #[test]
    fn desugar_or_operation() {
        let create_module = |expression: Expression| {
            Module::from_definitions(vec![ValueDefinition::new(
                "x",
                expression,
                types::Boolean::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()])
        };

        assert_eq!(
            desugar_with_types(&create_module(
                Operation::with_type(
                    types::Boolean::new(SourceInformation::dummy()),
                    Operator::Or,
                    Boolean::new(false, SourceInformation::dummy()),
                    Boolean::new(false, SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()
            )),
            Ok(create_module(
                If::new(
                    Boolean::new(false, SourceInformation::dummy()),
                    Boolean::new(true, SourceInformation::dummy()),
                    Boolean::new(false, SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into(),
            ))
        );
    }
}
