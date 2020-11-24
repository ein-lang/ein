mod boolean_compiler;
mod compile_configuration;
mod error;
mod expression_compiler;
mod expression_type_extractor;
mod global_name_map_creator;
mod global_name_renamer;
mod last_result_type_calculator;
mod list_type_configuration;
mod main_function_definition_transformer;
mod module_compiler;
mod module_environment_creator;
mod module_interface_compiler;
mod name_generator;
mod none_compiler;
mod reference_type_resolver;
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
use expression_compiler::{ExpressionCompiler, ExpressionCompilerSet, ExpressionTransformerSet};
use global_name_map_creator::GlobalNameMapCreator;
use global_name_renamer::GlobalNameRenamer;
use last_result_type_calculator::LastResultTypeCalculator;
pub use list_type_configuration::ListTypeConfiguration;
use main_function_definition_transformer::MainFunctionDefinitionTransformer;
use module_compiler::ModuleCompiler;
use module_interface_compiler::ModuleInterfaceCompiler;
use none_compiler::NoneCompiler;
use reference_type_resolver::ReferenceTypeResolver;
use std::sync::Arc;
use transform::{
    transform_before_name_qualification, transform_with_types, transform_without_types,
    BooleanOperationTransformer, EqualOperationTransformer, FunctionTypeCoercionTransformer,
    ListCaseTransformer, ListLiteralTransformer, NotEqualOperationTransformer,
};
use type_comparability_checker::TypeComparabilityChecker;
use type_compiler::TypeCompiler;
use type_equality_checker::TypeEqualityChecker;
use type_inference::infer_types;
use union_tag_calculator::UnionTagCalculator;
use variable_compiler::VariableCompiler;

// TODO Use Arc for compile_configuration.
pub fn compile(
    module: &Module,
    configuration: &CompileConfiguration,
) -> Result<(Vec<u8>, ModuleInterface), CompileError> {
    let module = transform_before_name_qualification(&module)?;

    let names = GlobalNameMapCreator::create(&module);
    let module = GlobalNameRenamer::new(names.clone()).rename(&module);
    let module =
        MainFunctionDefinitionTransformer::new(names.clone(), configuration.clone().into())
            .transform(&module);

    let list_type_configuration = Arc::new(configuration.list_type_configuration().qualify(&names));
    let module = transform_with_types(&infer_types(&transform_without_types(&module)?)?)?;

    let reference_type_resolver = ReferenceTypeResolver::new(&module);
    let type_comparability_checker = TypeComparabilityChecker::new(reference_type_resolver.clone());
    let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());
    let last_result_type_calculator =
        LastResultTypeCalculator::new(reference_type_resolver.clone());
    let union_tag_calculator = UnionTagCalculator::new(reference_type_resolver.clone());
    let type_compiler = TypeCompiler::new(
        reference_type_resolver.clone(),
        union_tag_calculator.clone(),
        list_type_configuration.clone(),
    );
    let boolean_compiler = BooleanCompiler::new(type_compiler.clone());
    let none_compiler = NoneCompiler::new(type_compiler.clone());
    let variable_compiler = VariableCompiler::new(type_compiler.clone(), &module);

    let equal_operation_transformer = EqualOperationTransformer::new(
        reference_type_resolver.clone(),
        type_comparability_checker,
        type_equality_checker.clone(),
        list_type_configuration.clone(),
    );
    let not_equal_operation_transformer = NotEqualOperationTransformer::new();
    let list_literal_transformer = ListLiteralTransformer::new(list_type_configuration.clone());
    let boolean_operation_transformer = BooleanOperationTransformer::new();
    let function_type_coercion_transformer = FunctionTypeCoercionTransformer::new(
        type_equality_checker,
        reference_type_resolver.clone(),
    );
    let list_case_transformer =
        ListCaseTransformer::new(reference_type_resolver.clone(), list_type_configuration);

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
            list_case_transformer,
        }
        .into(),
        reference_type_resolver,
        last_result_type_calculator,
        union_tag_calculator,
        type_compiler.clone(),
    );

    Ok((
        ssf_llvm::compile(
            &ModuleCompiler::new(expression_compiler, type_compiler).compile(&module)?,
            ssf_llvm::CompileConfiguration::new(
                Some(configuration.malloc_function_name().into()),
                Some(configuration.panic_function_name().into()),
            ),
        )?,
        ModuleInterfaceCompiler::new().compile(&module)?,
    ))
}

#[cfg(test)]
mod tests {
    use super::list_type_configuration::LIST_TYPE_CONFIGURATION;
    use super::*;
    use crate::debug::*;
    use crate::types;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref COMPILE_CONFIGURATION: CompileConfiguration = CompileConfiguration::new(
            "main",
            "ein_main",
            "ein_malloc",
            "ein_panic",
            LIST_TYPE_CONFIGURATION.clone()
        );
    }

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
                    Operation::new(
                        Operator::Add,
                        Variable::new("x", SourceInformation::dummy()),
                        Number::new(42.0, SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()
            ]),
            &COMPILE_CONFIGURATION
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
            &COMPILE_CONFIGURATION,
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
            &COMPILE_CONFIGURATION,
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
            &COMPILE_CONFIGURATION,
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
            &COMPILE_CONFIGURATION,
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
            &COMPILE_CONFIGURATION,
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
            &COMPILE_CONFIGURATION,
        )
        .unwrap();
    }

    #[test]
    fn compile_equal_operation_with_none_type() {
        compile(
            &Module::from_definitions(vec![VariableDefinition::new(
                "x",
                Operation::new(
                    Operator::Equal,
                    None::new(SourceInformation::dummy()),
                    None::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                types::Boolean::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()]),
            &COMPILE_CONFIGURATION,
        )
        .unwrap();
    }

    #[test]
    fn compile_equal_operation_with_boolean_type() {
        compile(
            &Module::from_definitions(vec![VariableDefinition::new(
                "x",
                Operation::new(
                    Operator::Equal,
                    Boolean::new(false, SourceInformation::dummy()),
                    Boolean::new(true, SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                types::Boolean::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()]),
            &COMPILE_CONFIGURATION,
        )
        .unwrap();
    }

    #[test]
    fn compile_equal_operation_with_union_type() {
        compile(
            &Module::from_definitions(vec![VariableDefinition::new(
                "x",
                Operation::new(
                    Operator::Equal,
                    None::new(SourceInformation::dummy()),
                    None::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                types::Boolean::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()]),
            &COMPILE_CONFIGURATION,
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
            &COMPILE_CONFIGURATION,
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
            &COMPILE_CONFIGURATION,
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
            &COMPILE_CONFIGURATION,
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
            &COMPILE_CONFIGURATION,
        )
        .unwrap();
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
}
