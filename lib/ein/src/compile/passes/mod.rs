mod boolean_operation_pass;
mod elementless_record_pass;
mod equal_operation_pass;
mod function_type_argument_pass;
mod list_literal_pass;
mod list_type_pass;
mod not_equal_operation_pass;
mod partial_application_pass;
mod record_function_pass;
mod record_update_pass;
mod type_coercion_pass;
mod typed_meta_pass;

use super::error::CompileError;
use super::expression_type_extractor::ExpressionTypeExtractor;
use super::list_literal_configuration::ListLiteralConfiguration;
use super::reference_type_resolver::ReferenceTypeResolver;
use super::type_equality_checker::TypeEqualityChecker;
use super::union_type_simplifier::UnionTypeSimplifier;
use crate::ast::*;
pub use boolean_operation_pass::BooleanOperationPass;
pub use elementless_record_pass::ElementlessRecordPass;
pub use equal_operation_pass::EqualOperationPass;
pub use function_type_argument_pass::FunctionTypeArgumentPass;
pub use list_literal_pass::ListLiteralPass;
pub use list_type_pass::ListTypePass;
pub use not_equal_operation_pass::NotEqualOperationPass;
pub use partial_application_pass::PartialApplicationPass;
pub use record_function_pass::RecordFunctionPass;
pub use record_update_pass::RecordUpdatePass;
use std::sync::Arc;
pub use type_coercion_pass::TypeCoercionPass;
pub use typed_meta_pass::TypedMetaPass;

pub fn compile_without_types(module: &Module) -> Result<Module, CompileError> {
    RecordUpdatePass::new().compile(module)
}

pub fn compile_with_types(
    module: &Module,
    list_literal_configuration: Arc<ListLiteralConfiguration>,
) -> Result<Module, CompileError> {
    let reference_type_resolver = ReferenceTypeResolver::new(module);
    let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());
    let union_type_simplifier = UnionTypeSimplifier::new(
        reference_type_resolver.clone(),
        type_equality_checker.clone(),
    );
    let expression_type_extractor = ExpressionTypeExtractor::new(
        reference_type_resolver.clone(),
        union_type_simplifier.clone(),
    );

    let module = ListLiteralPass::new(list_literal_configuration.clone()).compile(&module)?;
    let module = BooleanOperationPass::new().compile(&module)?;

    let module = NotEqualOperationPass::new().compile(&module)?;
    let module = EqualOperationPass::new(
        reference_type_resolver.clone(),
        type_equality_checker.clone(),
        list_literal_configuration.clone(),
    )
    .compile(&module)?;

    let module = TypedMetaPass::new(FunctionTypeArgumentPass::new(
        reference_type_resolver.clone(),
        type_equality_checker.clone(),
        expression_type_extractor.clone(),
    ))
    .compile(&module)?;

    let module = PartialApplicationPass::new().compile(&module)?;
    let module = ListTypePass::new(list_literal_configuration).compile(&module)?;

    TypedMetaPass::new(TypeCoercionPass::new(
        reference_type_resolver,
        type_equality_checker,
        expression_type_extractor,
        union_type_simplifier,
    ))
    .compile(&module)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug::SourceInformation;
    use crate::path::ModulePath;
    use crate::types;
    use insta::assert_debug_snapshot;
    use pretty_assertions::assert_eq;

    fn list_module_interface() -> ModuleInterface {
        ModuleInterface::new(
            ModulePath::dummy(),
            vec!["empty", "concatenate", "prepend"]
                .into_iter()
                .map(String::from)
                .collect(),
            vec![(
                "GenericList".into(),
                types::Record::new(
                    "GenericList",
                    Default::default(),
                    SourceInformation::dummy(),
                )
                .into(),
            )]
            .into_iter()
            .collect(),
            vec![
                (
                    "empty".into(),
                    types::Reference::new("GenericList", SourceInformation::dummy()).into(),
                ),
                (
                    "concatenate".into(),
                    types::Function::new(
                        types::Reference::new("GenericList", SourceInformation::dummy()),
                        types::Function::new(
                            types::Reference::new("GenericList", SourceInformation::dummy()),
                            types::Reference::new("GenericList", SourceInformation::dummy()),
                            SourceInformation::dummy(),
                        ),
                        SourceInformation::dummy(),
                    )
                    .into(),
                ),
                (
                    "equal".into(),
                    types::Function::new(
                        types::Function::new(
                            types::Any::new(SourceInformation::dummy()),
                            types::Function::new(
                                types::Any::new(SourceInformation::dummy()),
                                types::Boolean::new(SourceInformation::dummy()),
                                SourceInformation::dummy(),
                            ),
                            SourceInformation::dummy(),
                        ),
                        types::Function::new(
                            types::Reference::new("GenericList", SourceInformation::dummy()),
                            types::Function::new(
                                types::Reference::new("GenericList", SourceInformation::dummy()),
                                types::Boolean::new(SourceInformation::dummy()),
                                SourceInformation::dummy(),
                            ),
                            SourceInformation::dummy(),
                        ),
                        SourceInformation::dummy(),
                    )
                    .into(),
                ),
                (
                    "prepend".into(),
                    types::Function::new(
                        types::Any::new(SourceInformation::dummy()),
                        types::Function::new(
                            types::Reference::new("GenericList", SourceInformation::dummy()),
                            types::Reference::new("GenericList", SourceInformation::dummy()),
                            SourceInformation::dummy(),
                        ),
                        SourceInformation::dummy(),
                    )
                    .into(),
                ),
            ]
            .into_iter()
            .collect(),
        )
    }

    fn compile_with_types(module: &Module) -> Result<Module, CompileError> {
        super::compile_with_types(
            module,
            ListLiteralConfiguration::new(
                "empty",
                "concatenate",
                "equal",
                "prepend",
                "GenericList",
            )
            .into(),
        )
    }

    mod type_coercion {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn compile_function_definition() {
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
                compile_with_types(&create_module(
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
        fn compile_value_definition() {
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
                compile_with_types(&create_module(
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
        fn compile_application() {
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
                compile_with_types(&create_module(
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
        fn compile_let_value_expression() {
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
                compile_with_types(&create_module(
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
        fn compile_let_function_expression() {
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
                compile_with_types(&create_module(
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
        fn compile_record_construction() {
            let union_type = types::Union::new(
                vec![
                    types::Number::new(SourceInformation::dummy()).into(),
                    types::None::new(SourceInformation::dummy()).into(),
                ],
                SourceInformation::dummy(),
            );
            let reference_type = types::Reference::new("Foo", SourceInformation::dummy());

            assert_debug_snapshot!(compile_with_types(
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
        fn compile_union() {
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
                compile_with_types(&create_module(
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
        fn compile_function() {
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
                compile_with_types(&create_module(
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
        fn compile_function_as_argument() {
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
                compile_with_types(&create_module(
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

        #[test]
        fn compile_any() {
            let create_module = |expression: Expression| {
                Module::from_definitions(vec![ValueDefinition::new(
                    "x",
                    expression,
                    types::Any::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()])
            };

            assert_eq!(
                compile_with_types(&create_module(
                    Number::new(42.0, SourceInformation::dummy()).into()
                )),
                Ok(create_module(
                    TypeCoercion::new(
                        Number::new(42.0, SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        types::Any::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    )
                    .into()
                ))
            );
        }
    }

    #[test]
    fn compile_case_expression() {
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

        assert_debug_snapshot!(compile_with_types(&Module::from_definitions(vec![
            ValueDefinition::new(
                "x",
                Case::with_type(
                    argument_union_type.clone(),
                    "foo",
                    Boolean::new(false, SourceInformation::dummy()),
                    vec![
                        Alternative::new(
                            types::Boolean::new(SourceInformation::dummy()),
                            Number::new(42.0, SourceInformation::dummy()),
                        ),
                        Alternative::new(
                            types::None::new(SourceInformation::dummy()),
                            None::new(SourceInformation::dummy()),
                        ),
                    ],
                    SourceInformation::dummy(),
                ),
                result_union_type.clone(),
                SourceInformation::dummy(),
            )
            .into()
        ])));
    }

    mod equal_operations {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn compile_union_equal_operation() {
            let union_type = types::Union::new(
                vec![
                    types::Number::new(SourceInformation::dummy()).into(),
                    types::None::new(SourceInformation::dummy()).into(),
                ],
                SourceInformation::dummy(),
            );

            assert_debug_snapshot!(compile_with_types(&Module::from_definitions(vec![
                ValueDefinition::new(
                    "x",
                    Operation::with_type(
                        union_type.clone(),
                        Operator::Equal,
                        None::new(SourceInformation::dummy()),
                        None::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    types::Boolean::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()
            ])));
        }

        #[test]
        fn compile_list_equal_operation() {
            let list_type = types::List::new(
                types::None::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            );

            assert_debug_snapshot!(compile_with_types(&Module::new(
                ModulePath::dummy(),
                Export::new(Default::default()),
                vec![Import::new(list_module_interface(), false)],
                vec![],
                vec![ValueDefinition::new(
                    "x",
                    Operation::with_type(
                        list_type.clone(),
                        Operator::Equal,
                        List::with_type(list_type.clone(), vec![], SourceInformation::dummy()),
                        List::with_type(list_type.clone(), vec![], SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    types::Boolean::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()]
            )));
        }

        #[test]
        fn compile_not_equal_operation() {
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
                compile_with_types(&create_module(
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
    }

    #[test]
    fn compile_and_operation() {
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
            compile_with_types(&create_module(
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
    fn compile_or_operation() {
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
            compile_with_types(&create_module(
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

    mod list_literals {
        use super::*;

        #[test]
        fn compile_empty_list() {
            let list_type = types::List::new(
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            );

            assert_debug_snapshot!(compile_with_types(&Module::new(
                ModulePath::dummy(),
                Export::new(Default::default()),
                vec![Import::new(list_module_interface(), false)],
                vec![],
                vec![ValueDefinition::new(
                    "x",
                    List::with_type(list_type.clone(), vec![], SourceInformation::dummy(),),
                    list_type.clone(),
                    SourceInformation::dummy(),
                )
                .into()]
            )));
        }

        #[test]
        fn compile_list_with_an_element() {
            let list_type = types::List::new(
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            );

            assert_debug_snapshot!(compile_with_types(&Module::new(
                ModulePath::dummy(),
                Export::new(Default::default()),
                vec![Import::new(list_module_interface(), false)],
                vec![],
                vec![ValueDefinition::new(
                    "x",
                    List::with_type(
                        list_type.clone(),
                        vec![ListElement::Single(
                            Number::new(42.0, SourceInformation::dummy()).into()
                        )],
                        SourceInformation::dummy(),
                    ),
                    list_type.clone(),
                    SourceInformation::dummy(),
                )
                .into()]
            )));
        }

        #[test]
        fn compile_list_with_spread_element() {
            let list_type = types::List::new(
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            );

            assert_debug_snapshot!(compile_with_types(&Module::new(
                ModulePath::dummy(),
                Export::new(Default::default()),
                vec![Import::new(list_module_interface(), false)],
                vec![],
                vec![ValueDefinition::new(
                    "x",
                    List::with_type(
                        list_type.clone(),
                        vec![ListElement::Multiple(
                            List::new(vec![], SourceInformation::dummy(),).into()
                        )],
                        SourceInformation::dummy(),
                    ),
                    list_type,
                    SourceInformation::dummy(),
                )
                .into()]
            )));
        }
    }
}
