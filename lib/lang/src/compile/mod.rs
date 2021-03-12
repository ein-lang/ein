mod boolean_compiler;
mod compile_configuration;
mod error;
mod error_type_configuration;
mod expression_compiler;
mod expression_type_extractor;
mod global_name_map_creator;
mod global_name_renamer;
mod global_name_validator;
mod last_result_type_calculator;
mod list_type_configuration;
mod main_function_definition_transformer;
mod main_module_configuration;
mod module_compiler;
mod module_environment_creator;
mod module_interface_compiler;
mod name_generator;
mod none_compiler;
mod reference_type_resolver;
mod string_type_configuration;
mod transform;
mod type_canonicalizer;
mod type_comparability_checker;
mod type_compiler;
mod type_equality_checker;
mod type_inference;
mod union_tag_calculator;
mod variable_compiler;

use crate::ast::*;
use boolean_compiler::BooleanCompiler;
pub use compile_configuration::CompileConfiguration;
use error::CompileError;
pub use error_type_configuration::ErrorTypeConfiguration;
use expression_compiler::{ExpressionCompiler, ExpressionCompilerSet, ExpressionTransformerSet};
use global_name_map_creator::GlobalNameMapCreator;
use global_name_renamer::GlobalNameRenamer;
use global_name_validator::GlobalNameValidator;
use last_result_type_calculator::LastResultTypeCalculator;
pub use list_type_configuration::ListTypeConfiguration;
use main_function_definition_transformer::MainFunctionDefinitionTransformer;
pub use main_module_configuration::MainModuleConfiguration;
use module_compiler::ModuleCompiler;
use module_interface_compiler::ModuleInterfaceCompiler;
use none_compiler::NoneCompiler;
use reference_type_resolver::ReferenceTypeResolver;
use std::sync::Arc;
pub use string_type_configuration::StringTypeConfiguration;
use transform::{
    transform_before_name_qualification, transform_with_types, transform_without_types,
    BooleanOperationTransformer, EqualOperationTransformer, FunctionTypeCoercionTransformer,
    LetErrorTransformer, ListCaseTransformer, ListLiteralTransformer, ListTypeCoercionTransformer,
    NotEqualOperationTransformer,
};
use type_canonicalizer::TypeCanonicalizer;
use type_comparability_checker::TypeComparabilityChecker;
use type_compiler::TypeCompiler;
use type_equality_checker::TypeEqualityChecker;
use type_inference::infer_types;
use union_tag_calculator::UnionTagCalculator;
use variable_compiler::VariableCompiler;

pub fn compile(
    module: &Module,
    configuration: Arc<CompileConfiguration>,
) -> Result<(Vec<u8>, ModuleInterface), CompileError> {
    GlobalNameValidator::new().validate(&module)?;

    let module = transform_before_name_qualification(&module)?;

    let module = if let Some(main_module_configuration) = &configuration.main_module_configuration {
        MainFunctionDefinitionTransformer::new(main_module_configuration.clone())
            .transform(&module)?
    } else {
        module
    };

    let global_names = GlobalNameMapCreator::create(&module);
    let configuration = Arc::new(configuration.qualify(&global_names));
    let module = GlobalNameRenamer::new(global_names.clone()).rename(&module);

    let module = transform_with_types(
        &infer_types(&transform_without_types(&module)?, configuration.clone())?,
        configuration.clone(),
    )?;

    let reference_type_resolver = ReferenceTypeResolver::new(&module);
    let type_comparability_checker = TypeComparabilityChecker::new(reference_type_resolver.clone());
    let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());
    let type_canonicalizer = TypeCanonicalizer::new(
        reference_type_resolver.clone(),
        type_equality_checker.clone(),
    );
    let last_result_type_calculator =
        LastResultTypeCalculator::new(reference_type_resolver.clone());
    let union_tag_calculator = UnionTagCalculator::new(reference_type_resolver.clone());
    let type_compiler = TypeCompiler::new(
        reference_type_resolver.clone(),
        union_tag_calculator.clone(),
        configuration.list_type_configuration.clone(),
    );
    let boolean_compiler = BooleanCompiler::new(type_compiler.clone());
    let none_compiler = NoneCompiler::new(type_compiler.clone());
    let variable_compiler = VariableCompiler::new(
        type_compiler.clone(),
        reference_type_resolver.clone(),
        &module,
    )?;

    let equal_operation_transformer = EqualOperationTransformer::new(
        reference_type_resolver.clone(),
        type_comparability_checker,
        type_equality_checker.clone(),
        configuration.list_type_configuration.clone(),
    );
    let not_equal_operation_transformer = NotEqualOperationTransformer::new();
    let list_literal_transformer = ListLiteralTransformer::new(
        reference_type_resolver.clone(),
        configuration.list_type_configuration.clone(),
    );
    let boolean_operation_transformer = BooleanOperationTransformer::new();
    let function_type_coercion_transformer = FunctionTypeCoercionTransformer::new(
        type_equality_checker.clone(),
        reference_type_resolver.clone(),
    );
    let list_type_coercion_transformer = ListTypeCoercionTransformer::new(
        type_equality_checker.clone(),
        reference_type_resolver.clone(),
        configuration.list_type_configuration.clone(),
    );
    let list_case_transformer = ListCaseTransformer::new(
        reference_type_resolver.clone(),
        configuration.list_type_configuration.clone(),
    );
    let let_error_transformer = LetErrorTransformer::new(
        reference_type_resolver.clone(),
        type_equality_checker,
        type_canonicalizer,
        configuration.error_type_configuration.clone(),
    );

    let expression_compiler = ExpressionCompiler::new(
        ExpressionCompilerSet {
            boolean_compiler,
            none_compiler,
            variable_compiler,
        }
        .into(),
        ExpressionTransformerSet {
            equal_operation_transformer,
            not_equal_operation_transformer,
            list_literal_transformer,
            boolean_operation_transformer,
            function_type_coercion_transformer,
            list_type_coercion_transformer,
            list_case_transformer,
            let_error_transformer,
        }
        .into(),
        reference_type_resolver,
        last_result_type_calculator,
        union_tag_calculator,
        type_compiler.clone(),
        configuration.string_type_configuration.clone(),
    );

    let fmm_module = fmm::analysis::transform_to_cps(
        &ssf_fmm::compile(
            &ModuleCompiler::new(expression_compiler, type_compiler, global_names)
                .compile(&module)?,
        ),
        fmm::types::Record::new(vec![]),
    )
    .unwrap();

    fmm::analysis::check_types(&fmm_module).unwrap();

    Ok((
        fmm_c::compile(
            &fmm_module,
            Some(fmm_c::MallocConfiguration {
                malloc_function_name: configuration.malloc_function_name.clone(),
                realloc_function_name: configuration.realloc_function_name.clone(),
            }),
        )
        .into(),
        ModuleInterfaceCompiler::new().compile(&module)?,
    ))
}

#[cfg(test)]
mod tests {
    use super::compile_configuration::COMPILE_CONFIGURATION;
    use super::*;
    use crate::debug::*;
    use crate::path::ModulePath;
    use crate::types;

    #[test]
    fn compile_constant_initialized_with_operation() {
        assert!(compile(
            &Module::from_definitions(vec![
                VariableDefinition::new(
                    "x",
                    Number::new(42.0, SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into(),
                VariableDefinition::new(
                    "y",
                    ArithmeticOperation::new(
                        ArithmeticOperator::Add,
                        Variable::new("x", SourceInformation::dummy()),
                        Number::new(42.0, SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()
            ]),
            COMPILE_CONFIGURATION.clone(),
        )
        .is_ok());
    }

    #[test]
    fn compile_record_construction() {
        let reference_type = types::Reference::new("Foo", SourceInformation::dummy());

        compile(
            &Module::from_definitions_and_type_definitions(
                vec![TypeDefinition::new(
                    "Foo",
                    types::Record::new(
                        "Foo",
                        vec![(
                            "foo".into(),
                            types::Number::new(SourceInformation::dummy()).into(),
                        )]
                        .into_iter()
                        .collect(),
                        SourceInformation::dummy(),
                    ),
                )],
                vec![VariableDefinition::new(
                    "x",
                    RecordConstruction::new(
                        reference_type.clone(),
                        vec![(
                            "foo".into(),
                            Number::new(42.0, SourceInformation::dummy()).into(),
                        )]
                        .into_iter()
                        .collect(),
                        SourceInformation::dummy(),
                    ),
                    reference_type,
                    SourceInformation::dummy(),
                )
                .into()],
            ),
            COMPILE_CONFIGURATION.clone(),
        )
        .unwrap();
    }

    #[test]
    fn compile_record_element_access() {
        let reference_type = types::Reference::new("Foo", SourceInformation::dummy());

        compile(
            &Module::from_definitions_and_type_definitions(
                vec![TypeDefinition::new(
                    "Foo",
                    types::Record::new(
                        "Foo",
                        vec![(
                            "foo".into(),
                            types::Number::new(SourceInformation::dummy()).into(),
                        )]
                        .into_iter()
                        .collect(),
                        SourceInformation::dummy(),
                    ),
                )],
                vec![VariableDefinition::new(
                    "x",
                    Application::new(
                        Variable::new("Foo.foo", SourceInformation::dummy()),
                        RecordConstruction::new(
                            reference_type,
                            vec![(
                                "foo".into(),
                                Number::new(42.0, SourceInformation::dummy()).into(),
                            )]
                            .into_iter()
                            .collect(),
                            SourceInformation::dummy(),
                        ),
                        SourceInformation::dummy(),
                    ),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()],
            ),
            COMPILE_CONFIGURATION.clone(),
        )
        .unwrap();
    }

    #[test]
    fn compile_elementless_record_types() {
        let reference_type = types::Reference::new("Foo", SourceInformation::dummy());

        compile(
            &Module::from_definitions_and_type_definitions(
                vec![TypeDefinition::new(
                    "Foo",
                    types::Record::new("Foo", Default::default(), SourceInformation::dummy()),
                )],
                vec![VariableDefinition::new(
                    "x",
                    Variable::new("Foo", SourceInformation::dummy()),
                    reference_type,
                    SourceInformation::dummy(),
                )
                .into()],
            ),
            COMPILE_CONFIGURATION.clone(),
        )
        .unwrap();
    }

    #[test]
    fn compile_record_with_any_type_member() {
        compile(
            &Module::from_definitions_and_type_definitions(
                vec![TypeDefinition::new(
                    "Foo",
                    types::Record::new(
                        "Foo",
                        vec![(
                            "foo".into(),
                            types::Any::new(SourceInformation::dummy()).into(),
                        )]
                        .into_iter()
                        .collect(),
                        SourceInformation::dummy(),
                    ),
                )],
                vec![],
            ),
            COMPILE_CONFIGURATION.clone(),
        )
        .unwrap();
    }

    #[test]
    fn compile_case_expression() {
        compile(
            &Module::from_definitions(vec![VariableDefinition::new(
                "x",
                Case::new(
                    "x",
                    If::new(
                        Boolean::new(false, SourceInformation::dummy()),
                        Number::new(42.0, SourceInformation::dummy()),
                        None::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    vec![
                        Alternative::new(
                            types::Number::new(SourceInformation::dummy()),
                            Boolean::new(false, SourceInformation::dummy()),
                        ),
                        Alternative::new(
                            types::None::new(SourceInformation::dummy()),
                            None::new(SourceInformation::dummy()),
                        ),
                    ],
                    SourceInformation::dummy(),
                ),
                types::Union::new(
                    vec![
                        types::Boolean::new(SourceInformation::dummy()).into(),
                        types::None::new(SourceInformation::dummy()).into(),
                    ],
                    SourceInformation::dummy(),
                ),
                SourceInformation::dummy(),
            )
            .into()]),
            COMPILE_CONFIGURATION.clone(),
        )
        .unwrap();
    }

    #[test]
    fn compile_case_expression_with_union_type_alternative() {
        let union_type = types::Union::new(
            vec![
                types::Number::new(SourceInformation::dummy()).into(),
                types::None::new(SourceInformation::dummy()).into(),
            ],
            SourceInformation::dummy(),
        );

        compile(
            &Module::from_definitions(vec![VariableDefinition::new(
                "x",
                Case::new(
                    "y",
                    If::new(
                        Boolean::new(false, SourceInformation::dummy()),
                        Number::new(42.0, SourceInformation::dummy()),
                        None::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    vec![Alternative::new(
                        union_type.clone(),
                        Variable::new("y", SourceInformation::dummy()),
                    )],
                    SourceInformation::dummy(),
                ),
                union_type,
                SourceInformation::dummy(),
            )
            .into()]),
            COMPILE_CONFIGURATION.clone(),
        )
        .unwrap();
    }

    #[test]
    fn compile_equal_operation_with_none_type() {
        compile(
            &Module::from_definitions(vec![VariableDefinition::new(
                "x",
                EqualityOperation::new(
                    EqualityOperator::Equal,
                    None::new(SourceInformation::dummy()),
                    None::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                types::Boolean::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()]),
            COMPILE_CONFIGURATION.clone(),
        )
        .unwrap();
    }

    #[test]
    fn compile_equal_operation_with_boolean_type() {
        compile(
            &Module::from_definitions(vec![VariableDefinition::new(
                "x",
                EqualityOperation::new(
                    EqualityOperator::Equal,
                    Boolean::new(false, SourceInformation::dummy()),
                    Boolean::new(true, SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                types::Boolean::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()]),
            COMPILE_CONFIGURATION.clone(),
        )
        .unwrap();
    }

    #[test]
    fn compile_equal_operation_with_union_type() {
        compile(
            &Module::from_definitions(vec![VariableDefinition::new(
                "x",
                EqualityOperation::new(
                    EqualityOperator::Equal,
                    None::new(SourceInformation::dummy()),
                    None::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                types::Boolean::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()]),
            COMPILE_CONFIGURATION.clone(),
        )
        .unwrap();
    }

    #[test]
    fn compile_any_type() {
        compile(
            &Module::from_definitions(vec![VariableDefinition::new(
                "x",
                Number::new(42.0, SourceInformation::dummy()),
                types::Any::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()]),
            COMPILE_CONFIGURATION.clone(),
        )
        .unwrap();
    }

    #[test]
    fn compile_any_type_with_union_type() {
        compile(
            &Module::from_definitions(vec![VariableDefinition::new(
                "x",
                If::new(
                    Boolean::new(false, SourceInformation::dummy()),
                    Number::new(42.0, SourceInformation::dummy()),
                    None::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                types::Any::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()]),
            COMPILE_CONFIGURATION.clone(),
        )
        .unwrap();
    }

    #[test]
    fn compile_case_expression_with_any_type() {
        compile(
            &Module::from_definitions(vec![
                VariableDefinition::new(
                    "x",
                    Number::new(42.0, SourceInformation::dummy()),
                    types::Any::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into(),
                VariableDefinition::new(
                    "y",
                    Case::new(
                        "z",
                        Variable::new("x", SourceInformation::dummy()),
                        vec![
                            Alternative::new(
                                types::Number::new(SourceInformation::dummy()),
                                Variable::new("z", SourceInformation::dummy()),
                            ),
                            Alternative::new(
                                types::Union::new(
                                    vec![
                                        types::Boolean::new(SourceInformation::dummy()).into(),
                                        types::None::new(SourceInformation::dummy()).into(),
                                    ],
                                    SourceInformation::dummy(),
                                ),
                                Variable::new("z", SourceInformation::dummy()),
                            ),
                            Alternative::new(
                                types::Any::new(SourceInformation::dummy()),
                                Variable::new("z", SourceInformation::dummy()),
                            ),
                        ],
                        SourceInformation::dummy(),
                    ),
                    types::Any::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into(),
            ]),
            COMPILE_CONFIGURATION.clone(),
        )
        .unwrap();
    }

    #[test]
    fn compile_recursive_type_definition_not_normalized() {
        compile(
            &Module::from_definitions_and_type_definitions(
                vec![TypeDefinition::new(
                    "Foo",
                    types::Record::new(
                        "Foo",
                        vec![
                            (
                                "foo".into(),
                                types::Union::new(
                                    vec![
                                        types::Any::new(SourceInformation::dummy()).into(),
                                        types::None::new(SourceInformation::dummy()).into(),
                                    ],
                                    SourceInformation::dummy(),
                                )
                                .into(),
                            ),
                            (
                                "bar".into(),
                                types::Reference::new("Foo", SourceInformation::dummy()).into(),
                            ),
                        ]
                        .into_iter()
                        .collect(),
                        SourceInformation::dummy(),
                    ),
                )],
                vec![],
            ),
            COMPILE_CONFIGURATION.clone(),
        )
        .unwrap();
    }

    mod string {
        use super::*;

        #[test]
        fn compile_string() -> Result<(), CompileError> {
            compile(
                &Module::from_definitions(vec![VariableDefinition::new(
                    "x",
                    EinString::new("foo", SourceInformation::dummy()),
                    types::EinString::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()]),
                COMPILE_CONFIGURATION.clone(),
            )?;

            Ok(())
        }

        #[test]
        fn compile_string_in_union_type() {
            compile(
                &Module::from_definitions(vec![
                    VariableDefinition::new(
                        "x",
                        EinString::new("foo", SourceInformation::dummy()),
                        types::Union::new(
                            vec![
                                types::EinString::new(SourceInformation::dummy()).into(),
                                types::None::new(SourceInformation::dummy()).into(),
                            ],
                            SourceInformation::dummy(),
                        ),
                        SourceInformation::dummy(),
                    )
                    .into(),
                    VariableDefinition::new(
                        "y",
                        Case::new(
                            "z",
                            Variable::new("x", SourceInformation::dummy()),
                            vec![
                                Alternative::new(
                                    types::EinString::new(SourceInformation::dummy()),
                                    Variable::new("z", SourceInformation::dummy()),
                                ),
                                Alternative::new(
                                    types::None::new(SourceInformation::dummy()),
                                    EinString::new("", SourceInformation::dummy()),
                                ),
                            ],
                            SourceInformation::dummy(),
                        ),
                        types::EinString::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    )
                    .into(),
                ]),
                COMPILE_CONFIGURATION.clone(),
            )
            .unwrap();
        }

        #[test]
        fn compile_string_in_any_type() {
            compile(
                &Module::from_definitions(vec![
                    VariableDefinition::new(
                        "x",
                        EinString::new("foo", SourceInformation::dummy()),
                        types::Any::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    )
                    .into(),
                    VariableDefinition::new(
                        "y",
                        Case::new(
                            "z",
                            Variable::new("x", SourceInformation::dummy()),
                            vec![
                                Alternative::new(
                                    types::EinString::new(SourceInformation::dummy()),
                                    Variable::new("z", SourceInformation::dummy()),
                                ),
                                Alternative::new(
                                    types::Any::new(SourceInformation::dummy()),
                                    EinString::new("", SourceInformation::dummy()),
                                ),
                            ],
                            SourceInformation::dummy(),
                        ),
                        types::EinString::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    )
                    .into(),
                ]),
                COMPILE_CONFIGURATION.clone(),
            )
            .unwrap();
        }
    }

    // TODO Enable this test by importing a fake prelude module.
    // mod list {
    //     use super::*;

    //     #[test]
    //     fn compile_empty_list() -> Result<(), CompileError> {
    //         compile(
    //             &Module::from_definitions(vec![VariableDefinition::new(
    //                 "x",
    //                 List::new(vec![], SourceInformation::dummy()),
    //                 types::List::new(
    //                     types::Number::new(SourceInformation::dummy()),
    //                     SourceInformation::dummy(),
    //                 ),
    //                 SourceInformation::dummy(),
    //             )
    //             .into()]),
    //             &COMPILE_CONFIGURATION,
    //         )?;

    //         Ok(())
    //     }
    // }

    mod let_error {
        use super::*;
        use crate::package::Package;

        #[test]
        fn compile_let_error() {
            let union_type = types::Union::new(
                vec![
                    types::Number::new(SourceInformation::dummy()).into(),
                    types::Reference::new(
                        &COMPILE_CONFIGURATION
                            .error_type_configuration
                            .error_type_name,
                        SourceInformation::dummy(),
                    )
                    .into(),
                ],
                SourceInformation::dummy(),
            );

            let module = Module::new(
                ModulePath::new(Package::new("", ""), vec![]),
                Export::new(Default::default()),
                ExportForeign::new(Default::default()),
                vec![Import::new(
                    ModuleInterface::new(
                        ModulePath::new(Package::new("m", ""), vec![]),
                        Default::default(),
                        vec![(
                            "Error".into(),
                            types::Record::new(
                                "Error",
                                Default::default(),
                                SourceInformation::dummy(),
                            )
                            .into(),
                        )]
                        .into_iter()
                        .collect(),
                        Default::default(),
                    ),
                    false,
                )],
                vec![],
                vec![],
                vec![
                    VariableDefinition::new(
                        "x",
                        Number::new(42.0, SourceInformation::dummy()),
                        union_type.clone(),
                        SourceInformation::dummy(),
                    )
                    .into(),
                    VariableDefinition::new(
                        "y",
                        LetError::new(
                            vec![VariableDefinition::new(
                                "z",
                                Variable::new("x", SourceInformation::dummy()),
                                types::Variable::new(SourceInformation::dummy()),
                                SourceInformation::dummy(),
                            )],
                            ArithmeticOperation::new(
                                ArithmeticOperator::Add,
                                Variable::new("z", SourceInformation::dummy()),
                                Number::new(42.0, SourceInformation::dummy()),
                                SourceInformation::dummy(),
                            ),
                            SourceInformation::dummy(),
                        ),
                        union_type,
                        SourceInformation::dummy(),
                    )
                    .into(),
                ],
            );

            compile(&module, COMPILE_CONFIGURATION.clone()).unwrap();
        }

        #[test]
        fn compile_let_error_with_multiple_definitions() {
            let union_type = types::Union::new(
                vec![
                    types::Number::new(SourceInformation::dummy()).into(),
                    types::Reference::new(
                        &COMPILE_CONFIGURATION
                            .error_type_configuration
                            .error_type_name,
                        SourceInformation::dummy(),
                    )
                    .into(),
                ],
                SourceInformation::dummy(),
            );

            let module = Module::new(
                ModulePath::new(Package::new("", ""), vec![]),
                Export::new(Default::default()),
                ExportForeign::new(Default::default()),
                vec![Import::new(
                    ModuleInterface::new(
                        ModulePath::new(Package::new("m", ""), vec![]),
                        Default::default(),
                        vec![(
                            "Error".into(),
                            types::Record::new(
                                "Error",
                                Default::default(),
                                SourceInformation::dummy(),
                            )
                            .into(),
                        )]
                        .into_iter()
                        .collect(),
                        Default::default(),
                    ),
                    false,
                )],
                vec![],
                vec![],
                vec![
                    VariableDefinition::new(
                        "x",
                        Number::new(42.0, SourceInformation::dummy()),
                        union_type.clone(),
                        SourceInformation::dummy(),
                    )
                    .into(),
                    VariableDefinition::new(
                        "y",
                        LetError::new(
                            vec![
                                VariableDefinition::new(
                                    "z",
                                    Variable::new("x", SourceInformation::dummy()),
                                    types::Variable::new(SourceInformation::dummy()),
                                    SourceInformation::dummy(),
                                ),
                                VariableDefinition::new(
                                    "v",
                                    Variable::new("x", SourceInformation::dummy()),
                                    types::Variable::new(SourceInformation::dummy()),
                                    SourceInformation::dummy(),
                                ),
                            ],
                            ArithmeticOperation::new(
                                ArithmeticOperator::Add,
                                Variable::new("v", SourceInformation::dummy()),
                                Variable::new("z", SourceInformation::dummy()),
                                SourceInformation::dummy(),
                            ),
                            SourceInformation::dummy(),
                        ),
                        union_type,
                        SourceInformation::dummy(),
                    )
                    .into(),
                ],
            );

            compile(&module, COMPILE_CONFIGURATION.clone()).unwrap();
        }
    }

    #[test]
    fn compile_export_foreigns() {
        let module = Module::new(
            ModulePath::dummy(),
            Export::new(Default::default()),
            ExportForeign::new(vec!["foo".into()].into_iter().collect()),
            vec![],
            vec![],
            vec![],
            vec![VariableDefinition::new(
                "foo",
                Number::new(42.0, SourceInformation::dummy()),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()],
        );

        compile(&module, COMPILE_CONFIGURATION.clone()).unwrap();
    }
}
