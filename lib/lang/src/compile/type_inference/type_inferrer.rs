use super::{
    super::{
        error::CompileError, reference_type_resolver::ReferenceTypeResolver,
        type_canonicalizer::TypeCanonicalizer, type_equality_checker::TypeEqualityChecker,
    },
    constraint_checker::ConstraintChecker,
    constraint_collector::ConstraintCollector,
    constraint_solver::ConstraintSolver,
    variable_substitutor::VariableSubstitutor,
};
use crate::{
    ast::*,
    types::{self, Type},
};
use std::sync::Arc;

pub struct TypeInferrer {
    reference_type_resolver: Arc<ReferenceTypeResolver>,
    type_equality_checker: Arc<TypeEqualityChecker>,
    type_canonicalizer: Arc<TypeCanonicalizer>,
    constraint_collector: ConstraintCollector,
    constraint_solver: Arc<ConstraintSolver>,
}

impl TypeInferrer {
    pub fn new(
        reference_type_resolver: Arc<ReferenceTypeResolver>,
        type_equality_checker: Arc<TypeEqualityChecker>,
        type_canonicalizer: Arc<TypeCanonicalizer>,
        constraint_collector: ConstraintCollector,
        constraint_solver: Arc<ConstraintSolver>,
    ) -> Self {
        Self {
            reference_type_resolver,
            type_equality_checker,
            type_canonicalizer,
            constraint_collector,
            constraint_solver,
        }
    }

    pub fn infer(self, module: &Module) -> Result<Module, CompileError> {
        let module = module.transform_types(&mut |type_| -> Result<_, CompileError> {
            Ok(match type_ {
                Type::Unknown(unknown) => {
                    types::Variable::new(unknown.source_information().clone()).into()
                }
                _ => type_.clone(),
            })
        })?;

        let (solved_subsumption_set, mut checked_subsumption_set) =
            self.constraint_collector.collect(&module)?;

        let substitutions = self
            .constraint_solver
            .solve(solved_subsumption_set, &mut checked_subsumption_set)?;

        let substitutor = VariableSubstitutor::new(self.type_canonicalizer.clone(), substitutions);

        let checker = ConstraintChecker::new(
            substitutor.clone(),
            self.reference_type_resolver.clone(),
            self.type_equality_checker,
        );

        checker.check(checked_subsumption_set)?;

        module.transform_types(&mut |type_| substitutor.substitute(type_))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{
            super::{
                error_type_configuration::ERROR_TYPE_CONFIGURATION,
                module_environment_creator::ModuleEnvironmentCreator,
            },
            constraint_collector::ConstraintCollector,
            constraint_converter::ConstraintConverter,
        },
        *,
    };
    use crate::{
        debug::*,
        package::Package,
        path::*,
        types::{self, Type},
    };
    use insta::assert_debug_snapshot;
    use pretty_assertions::assert_eq;

    fn infer_types(module: &Module) -> Result<Module, CompileError> {
        let reference_type_resolver = ReferenceTypeResolver::new(&module);
        let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());
        let type_canonicalizer = TypeCanonicalizer::new(
            reference_type_resolver.clone(),
            type_equality_checker.clone(),
        );
        let constraint_converter = ConstraintConverter::new(reference_type_resolver.clone());
        let constraint_solver =
            ConstraintSolver::new(constraint_converter, reference_type_resolver.clone());
        let module_environment_creator = ModuleEnvironmentCreator::new();
        let constraint_collector = ConstraintCollector::new(
            reference_type_resolver.clone(),
            module_environment_creator,
            ERROR_TYPE_CONFIGURATION.clone(),
        );

        TypeInferrer::new(
            reference_type_resolver,
            type_equality_checker,
            type_canonicalizer,
            constraint_collector,
            constraint_solver,
        )
        .infer(module)
    }

    #[test]
    fn infer_types_with_empty_modules() {
        assert_eq!(
            infer_types(&Module::from_definitions(vec![])),
            Ok(Module::from_definitions(vec![]))
        );
    }

    #[test]
    fn infer_types_of_none_literals() {
        let module = Module::from_definitions(vec![VariableDefinition::new(
            "x",
            None::new(SourceInformation::dummy()),
            types::None::new(SourceInformation::dummy()),
            SourceInformation::dummy(),
        )
        .into()]);
        assert_eq!(infer_types(&module), Ok(module));
    }

    #[test]
    fn infer_types_of_variables() {
        let module = Module::from_definitions(vec![VariableDefinition::new(
            "x",
            Number::new(42.0, SourceInformation::dummy()),
            types::Number::new(SourceInformation::dummy()),
            SourceInformation::dummy(),
        )
        .into()]);
        assert_eq!(infer_types(&module), Ok(module));
    }

    #[test]
    fn fail_to_infer_types_of_variables() {
        let module = Module::from_definitions(vec![VariableDefinition::new(
            "x",
            Number::new(42.0, SourceInformation::dummy()),
            types::Function::new(
                types::Number::new(SourceInformation::dummy()),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            ),
            SourceInformation::dummy(),
        )
        .into()]);

        assert_eq!(
            infer_types(&module),
            Err(CompileError::TypesNotMatched(
                SourceInformation::dummy().into(),
                SourceInformation::dummy().into()
            ))
        );
    }

    #[test]
    fn infer_types_of_functions() {
        let module = Module::from_definitions(vec![FunctionDefinition::new(
            "f",
            vec!["x".into()],
            Number::new(42.0, SourceInformation::dummy()),
            types::Function::new(
                types::Number::new(SourceInformation::dummy()),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            ),
            SourceInformation::dummy(),
        )
        .into()]);

        assert_eq!(infer_types(&module), Ok(module));
    }

    #[test]
    fn fail_to_infer_types_of_functions() {
        let module = Module::from_definitions(vec![FunctionDefinition::new(
            "f",
            vec!["x".into()],
            Number::new(42.0, SourceInformation::dummy()),
            types::Function::new(
                types::Number::new(SourceInformation::dummy()),
                types::Function::new(
                    types::Number::new(SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                SourceInformation::dummy(),
            ),
            SourceInformation::dummy(),
        )
        .into()]);

        assert_eq!(
            infer_types(&module),
            Err(CompileError::TypesNotMatched(
                SourceInformation::dummy().into(),
                SourceInformation::dummy().into()
            ))
        );
    }

    #[test]
    fn infer_types_of_applications() {
        let module = Module::from_definitions(vec![
            FunctionDefinition::new(
                "f",
                vec!["x".into()],
                Number::new(42.0, SourceInformation::dummy()),
                types::Function::new(
                    types::Number::new(SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                SourceInformation::dummy(),
            )
            .into(),
            VariableDefinition::new(
                "x",
                Application::new(
                    Variable::new("f", SourceInformation::dummy()),
                    Number::new(42.0, SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into(),
        ]);

        assert_eq!(infer_types(&module), Ok(module));
    }

    #[test]
    fn fail_to_infer_types_of_applications() {
        let module = Module::from_definitions(vec![
            FunctionDefinition::new(
                "f",
                vec!["x".into()],
                Number::new(42.0, SourceInformation::dummy()),
                types::Function::new(
                    types::Number::new(SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                SourceInformation::dummy(),
            )
            .into(),
            VariableDefinition::new(
                "x",
                Application::new(
                    Application::new(
                        Variable::new("f", SourceInformation::dummy()),
                        Number::new(42.0, SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    Number::new(42.0, SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into(),
        ]);

        assert_eq!(
            infer_types(&module),
            Err(CompileError::TypesNotMatched(
                SourceInformation::dummy().into(),
                SourceInformation::dummy().into()
            ))
        );
    }

    #[test]
    fn infer_types_of_let() {
        let module = Module::from_definitions(vec![VariableDefinition::new(
            "x",
            Let::new(
                vec![VariableDefinition::new(
                    "y",
                    Number::new(42.0, SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )],
                Variable::new("y", SourceInformation::dummy()),
                SourceInformation::dummy(),
            ),
            types::Number::new(SourceInformation::dummy()),
            SourceInformation::dummy(),
        )
        .into()]);

        assert_eq!(infer_types(&module), Ok(module));
    }

    #[test]
    fn fail_to_infer_types_of_let() {
        let module = Module::from_definitions(vec![VariableDefinition::new(
            "x",
            Let::new(
                vec![VariableDefinition::new(
                    "y",
                    Number::new(42.0, SourceInformation::dummy()),
                    types::Function::new(
                        types::Number::new(SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    SourceInformation::dummy(),
                )],
                Variable::new("y", SourceInformation::dummy()),
                SourceInformation::dummy(),
            ),
            types::Number::new(SourceInformation::dummy()),
            SourceInformation::dummy(),
        )
        .into()]);

        assert_eq!(
            infer_types(&module),
            Err(CompileError::TypesNotMatched(
                SourceInformation::dummy().into(),
                SourceInformation::dummy().into()
            ))
        );
    }

    #[test]
    fn infer_types_of_let_recursive() {
        let module = Module::from_definitions(vec![VariableDefinition::new(
            "x",
            LetRecursive::new(
                vec![FunctionDefinition::new(
                    "f",
                    vec!["z".into()],
                    Variable::new("z", SourceInformation::dummy()),
                    types::Function::new(
                        types::Number::new(SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    SourceInformation::dummy(),
                )],
                Application::new(
                    Variable::new("f", SourceInformation::dummy()),
                    Number::new(42.0, SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                SourceInformation::dummy(),
            ),
            types::Number::new(SourceInformation::dummy()),
            SourceInformation::dummy(),
        )
        .into()]);

        assert_eq!(infer_types(&module), Ok(module));
    }

    #[test]
    fn fail_to_infer_types_of_let_recursive() {
        let module = Module::from_definitions(vec![VariableDefinition::new(
            "x",
            LetRecursive::new(
                vec![FunctionDefinition::new(
                    "f",
                    vec!["z".into()],
                    Variable::new("z", SourceInformation::dummy()),
                    types::Function::new(
                        types::Number::new(SourceInformation::dummy()),
                        types::Function::new(
                            types::Number::new(SourceInformation::dummy()),
                            types::Number::new(SourceInformation::dummy()),
                            SourceInformation::dummy(),
                        ),
                        SourceInformation::dummy(),
                    ),
                    SourceInformation::dummy(),
                )],
                Application::new(
                    Variable::new("f", SourceInformation::dummy()),
                    Number::new(42.0, SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                SourceInformation::dummy(),
            ),
            types::Number::new(SourceInformation::dummy()),
            SourceInformation::dummy(),
        )
        .into()]);

        assert_eq!(
            infer_types(&module),
            Err(CompileError::TypesNotMatched(
                SourceInformation::dummy().into(),
                SourceInformation::dummy().into()
            ))
        );
    }

    #[test]
    fn infer_types_of_let_with_type_variables() {
        assert_eq!(
            infer_types(&Module::from_definitions(vec![VariableDefinition::new(
                "x",
                Let::new(
                    vec![VariableDefinition::new(
                        "y",
                        Number::new(42.0, SourceInformation::dummy()),
                        types::Unknown::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )],
                    Variable::new("y", SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()])),
            Ok(Module::from_definitions(vec![VariableDefinition::new(
                "x",
                Let::new(
                    vec![VariableDefinition::new(
                        "y",
                        Number::new(42.0, SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),],
                    Variable::new("y", SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()]))
        );
    }

    #[test]
    fn infer_types_of_let_error() {
        let union_type = types::Union::new(
            vec![
                types::Number::new(SourceInformation::dummy()).into(),
                types::Reference::new(
                    &ERROR_TYPE_CONFIGURATION.error_type_name,
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
                        types::Record::new("Error", Default::default(), SourceInformation::dummy())
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
                            types::Unknown::new(SourceInformation::dummy()),
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

        insta::assert_debug_snapshot!(infer_types(&module));
    }

    #[test]
    fn fail_to_infer_types_with_missing_variables() {
        let module = Module::from_definitions(vec![VariableDefinition::new(
            "x",
            Variable::new("y", SourceInformation::dummy()),
            types::Number::new(SourceInformation::dummy()),
            SourceInformation::dummy(),
        )
        .into()]);

        assert_eq!(
            infer_types(&module),
            Err(CompileError::VariableNotFound(Variable::new(
                "y",
                SourceInformation::dummy()
            )))
        );
    }

    #[test]
    fn infer_types_of_variables_with_imports() {
        let module = Module::new(
            ModulePath::new(Package::new("", ""), vec![]),
            Export::new(Default::default()),
            ExportForeign::new(Default::default()),
            vec![Import::new(
                ModuleInterface::new(
                    ModulePath::new(Package::new("m", ""), vec![]),
                    Default::default(),
                    Default::default(),
                    vec![(
                        "x".into(),
                        types::Number::new(SourceInformation::dummy()).into(),
                    )]
                    .into_iter()
                    .collect(),
                ),
                true,
            )],
            vec![],
            vec![],
            vec![VariableDefinition::new(
                "y",
                Variable::new("x", SourceInformation::dummy()),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()],
        );
        assert_eq!(infer_types(&module), Ok(module));
    }

    #[test]
    fn infer_types_of_import_foreigns() {
        let module = Module::new(
            ModulePath::new(Package::new("", ""), vec![]),
            Export::new(Default::default()),
            ExportForeign::new(Default::default()),
            vec![],
            vec![ImportForeign::new(
                "f",
                "",
                CallingConvention::Native,
                types::Function::new(
                    types::Number::new(SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                SourceInformation::dummy(),
            )],
            vec![],
            vec![VariableDefinition::new(
                "x",
                Application::new(
                    Variable::new("f", SourceInformation::dummy()),
                    Number::new(42.0, SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()],
        );
        assert_eq!(infer_types(&module), Ok(module));
    }

    #[test]
    fn infer_types_of_let_with_recursive_functions_and_the_latter_typed() {
        assert_debug_snapshot!(infer_types(&Module::from_definitions(vec![
            VariableDefinition::new(
                "x",
                LetRecursive::new(
                    vec![
                        FunctionDefinition::new(
                            "f",
                            vec!["x".into()],
                            Application::new(
                                Variable::new("g", SourceInformation::dummy()),
                                Variable::new("x", SourceInformation::dummy()),
                                SourceInformation::dummy(),
                            ),
                            types::Unknown::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        ),
                        FunctionDefinition::new(
                            "g",
                            vec!["x".into()],
                            Application::new(
                                Variable::new("f", SourceInformation::dummy()),
                                Variable::new("x", SourceInformation::dummy()),
                                SourceInformation::dummy(),
                            ),
                            types::Function::new(
                                types::Number::new(SourceInformation::dummy()),
                                types::Number::new(SourceInformation::dummy()),
                                SourceInformation::dummy(),
                            ),
                            SourceInformation::dummy()
                        )
                    ],
                    Application::new(
                        Variable::new("f", SourceInformation::dummy()),
                        Number::new(42.0, SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    SourceInformation::dummy(),
                ),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()
        ])));
    }

    #[test]
    fn infer_types_of_let_with_recursive_functions_and_the_former_typed() {
        assert_debug_snapshot!(infer_types(&Module::from_definitions(vec![
            VariableDefinition::new(
                "x",
                LetRecursive::new(
                    vec![
                        FunctionDefinition::new(
                            "f",
                            vec!["x".into()],
                            Application::new(
                                Variable::new("g", SourceInformation::dummy()),
                                Variable::new("x", SourceInformation::dummy()),
                                SourceInformation::dummy(),
                            ),
                            types::Function::new(
                                types::Number::new(SourceInformation::dummy()),
                                types::Number::new(SourceInformation::dummy()),
                                SourceInformation::dummy(),
                            ),
                            SourceInformation::dummy()
                        ),
                        FunctionDefinition::new(
                            "g",
                            vec!["x".into()],
                            Application::new(
                                Variable::new("f", SourceInformation::dummy()),
                                Variable::new("x", SourceInformation::dummy()),
                                SourceInformation::dummy(),
                            ),
                            types::Unknown::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        )
                    ],
                    Application::new(
                        Variable::new("f", SourceInformation::dummy()),
                        Number::new(42.0, SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    SourceInformation::dummy(),
                ),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()
        ])));
    }

    #[test]
    fn infer_types_of_let_in_function_definition_with_recursive_functions() {
        assert_debug_snapshot!(infer_types(&Module::from_definitions(vec![
            FunctionDefinition::new(
                "f",
                vec!["x".into()],
                LetRecursive::new(
                    vec![
                        FunctionDefinition::new(
                            "f",
                            vec!["x".into()],
                            Application::new(
                                Variable::new("g", SourceInformation::dummy()),
                                Variable::new("x", SourceInformation::dummy()),
                                SourceInformation::dummy(),
                            ),
                            types::Unknown::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        ),
                        FunctionDefinition::new(
                            "g",
                            vec!["x".into()],
                            Application::new(
                                Variable::new("f", SourceInformation::dummy()),
                                Variable::new("x", SourceInformation::dummy()),
                                SourceInformation::dummy(),
                            ),
                            types::Function::new(
                                types::Number::new(SourceInformation::dummy()),
                                types::Number::new(SourceInformation::dummy()),
                                SourceInformation::dummy(),
                            ),
                            SourceInformation::dummy()
                        )
                    ],
                    Application::new(
                        Variable::new("f", SourceInformation::dummy()),
                        Number::new(42.0, SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    SourceInformation::dummy(),
                ),
                types::Function::new(
                    types::Number::new(SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                SourceInformation::dummy(),
            )
            .into()
        ])));
    }

    #[test]
    fn fail_to_infer_types_of_recursive_variable_definitions() {
        let module = Module::from_definitions(vec![VariableDefinition::new(
            "x",
            Let::new(
                vec![
                    VariableDefinition::new(
                        "a",
                        Variable::new("b", SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    VariableDefinition::new(
                        "b",
                        Variable::new("a", SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                ],
                Variable::new("a", SourceInformation::dummy()),
                SourceInformation::dummy(),
            ),
            types::Number::new(SourceInformation::dummy()),
            SourceInformation::dummy(),
        )
        .into()]);

        assert_eq!(
            infer_types(&module),
            Err(CompileError::VariableNotFound(Variable::new(
                "b",
                SourceInformation::dummy(),
            )))
        );
    }

    #[test]
    fn infer_types_with_reference_types() {
        let module = Module::from_definitions_and_type_definitions(
            vec![TypeDefinition::new(
                "Foo",
                types::Number::new(SourceInformation::dummy()),
            )],
            vec![VariableDefinition::new(
                "x",
                Number::new(42.0, SourceInformation::dummy()),
                types::Reference::new("Foo", SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()],
        );

        assert_debug_snapshot!(infer_types(&module));
    }

    #[test]
    fn fail_to_infer_types_with_reference_type_not_found() {
        let module = Module::from_definitions(vec![VariableDefinition::new(
            "x",
            Number::new(42.0, SourceInformation::dummy()),
            types::Reference::new("Foo", SourceInformation::dummy()),
            SourceInformation::dummy(),
        )
        .into()]);

        assert_eq!(
            infer_types(&module),
            Err(CompileError::TypeNotFound(types::Reference::new(
                "Foo",
                SourceInformation::dummy()
            )))
        );
    }

    #[test]
    fn infer_types_with_imported_reference_types() {
        let module = Module::new(
            ModulePath::new(Package::new("", ""), vec![]),
            Export::new(Default::default()),
            ExportForeign::new(Default::default()),
            vec![Import::new(
                ModuleInterface::new(
                    ModulePath::new(Package::new("P", ""), vec![]),
                    Default::default(),
                    vec![(
                        "Foo".into(),
                        types::Number::new(SourceInformation::dummy()).into(),
                    )]
                    .into_iter()
                    .collect(),
                    Default::default(),
                ),
                true,
            )],
            vec![],
            vec![],
            vec![VariableDefinition::new(
                "x",
                Number::new(42.0, SourceInformation::dummy()),
                types::Reference::new("Foo", SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()],
        );

        assert_debug_snapshot!(infer_types(&module));
    }

    #[test]
    fn infer_types_with_recursive_reference_types() {
        let module = Module::from_definitions_and_type_definitions(
            vec![TypeDefinition::new(
                "Foo",
                types::Function::new(
                    types::Reference::new("Foo", SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
            )],
            vec![
                FunctionDefinition::new(
                    "f",
                    vec!["g".into()],
                    Number::new(42.0, SourceInformation::dummy()),
                    types::Reference::new("Foo", SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into(),
                VariableDefinition::new(
                    "x",
                    Application::new(
                        Variable::new("f", SourceInformation::dummy()),
                        Variable::new("f", SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into(),
            ],
        );

        assert_debug_snapshot!(infer_types(&module));
    }

    #[test]
    fn fail_to_infer_types_with_invalid_union_type() {
        let module = Module::from_definitions(vec![
            VariableDefinition::new(
                "x",
                List::new(
                    vec![ListElement::Single(
                        Number::new(42.0, SourceInformation::dummy()).into(),
                    )],
                    SourceInformation::dummy(),
                ),
                types::Union::new(
                    vec![
                        types::List::new(
                            types::Number::new(SourceInformation::dummy()),
                            SourceInformation::dummy(),
                        )
                        .into(),
                        types::List::new(
                            types::None::new(SourceInformation::dummy()),
                            SourceInformation::dummy(),
                        )
                        .into(),
                    ],
                    SourceInformation::dummy(),
                ),
                SourceInformation::dummy(),
            )
            .into(),
            VariableDefinition::new(
                "y",
                Variable::new("x", SourceInformation::dummy()),
                types::List::new(
                    types::Union::new(
                        vec![
                            types::Number::new(SourceInformation::dummy()).into(),
                            types::None::new(SourceInformation::dummy()).into(),
                        ],
                        SourceInformation::dummy(),
                    ),
                    SourceInformation::dummy(),
                ),
                SourceInformation::dummy(),
            )
            .into(),
        ]);

        assert_debug_snapshot!(infer_types(&module));
    }

    mod if_ {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn infer_types_of_if_expressions() {
            let module = Module::from_definitions(vec![VariableDefinition::new(
                "x",
                If::new(
                    Boolean::new(true, SourceInformation::dummy()),
                    None::new(SourceInformation::dummy()),
                    None::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                types::None::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()]);
            assert_eq!(infer_types(&module), Ok(module));
        }

        #[test]
        fn fail_to_infer_types_of_if_expressions_with_invalid_condition_type() {
            let module = Module::from_definitions(vec![VariableDefinition::new(
                "x",
                If::new(
                    None::new(SourceInformation::dummy()),
                    None::new(SourceInformation::dummy()),
                    None::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                types::None::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()]);
            assert_eq!(
                infer_types(&module),
                Err(CompileError::TypesNotMatched(
                    SourceInformation::dummy().into(),
                    SourceInformation::dummy().into()
                ))
            );
        }

        #[test]
        fn fail_to_infer_types_of_if_expressions_with_unmatched_branch_types() {
            let module = Module::from_definitions(vec![VariableDefinition::new(
                "x",
                If::new(
                    Boolean::new(true, SourceInformation::dummy()),
                    Boolean::new(true, SourceInformation::dummy()),
                    None::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                types::None::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()]);
            assert_eq!(
                infer_types(&module),
                Err(CompileError::TypesNotMatched(
                    SourceInformation::dummy().into(),
                    SourceInformation::dummy().into()
                ))
            );
        }

        #[test]
        fn infer_types_of_if_expressions_with_unmatched_branch_types_in_let_expressions() {
            let create_module = |type_: Type| {
                Module::from_definitions(vec![VariableDefinition::new(
                    "x",
                    Let::new(
                        vec![VariableDefinition::new(
                            "y",
                            If::new(
                                Boolean::new(true, SourceInformation::dummy()),
                                Boolean::new(true, SourceInformation::dummy()),
                                None::new(SourceInformation::dummy()),
                                SourceInformation::dummy(),
                            ),
                            type_,
                            SourceInformation::dummy(),
                        )],
                        Number::new(42.0, SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()])
            };

            assert_eq!(
                infer_types(&create_module(
                    types::Unknown::new(SourceInformation::dummy()).into()
                )),
                Ok(create_module(
                    types::Union::new(
                        vec![
                            types::Boolean::new(SourceInformation::dummy()).into(),
                            types::None::new(SourceInformation::dummy()).into(),
                        ],
                        SourceInformation::dummy(),
                    )
                    .into()
                ))
            );
        }

        #[test]
        fn infer_types_of_nested_if_expressions_with_unmatched_branch_types_in_let_expressions() {
            let create_module = |type_: Type| {
                Module::from_definitions(vec![VariableDefinition::new(
                    "x",
                    Let::new(
                        vec![VariableDefinition::new(
                            "y",
                            If::new(
                                Boolean::new(true, SourceInformation::dummy()),
                                Boolean::new(true, SourceInformation::dummy()),
                                If::new(
                                    Boolean::new(true, SourceInformation::dummy()),
                                    Number::new(42.0, SourceInformation::dummy()),
                                    None::new(SourceInformation::dummy()),
                                    SourceInformation::dummy(),
                                ),
                                SourceInformation::dummy(),
                            ),
                            type_,
                            SourceInformation::dummy(),
                        )],
                        Number::new(42.0, SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()])
            };

            assert_eq!(
                infer_types(&create_module(
                    types::Unknown::new(SourceInformation::dummy()).into()
                )),
                Ok(create_module(
                    types::Union::new(
                        vec![
                            types::Boolean::new(SourceInformation::dummy()).into(),
                            types::None::new(SourceInformation::dummy()).into(),
                            types::Number::new(SourceInformation::dummy()).into(),
                        ],
                        SourceInformation::dummy(),
                    )
                    .into()
                ))
            );
        }
    }

    mod case {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn infer_types_of_case_expressions() {
            let create_module = |type_: Type| {
                Module::from_definitions(vec![VariableDefinition::new(
                    "x",
                    Case::with_type(
                        type_,
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
                .into()])
            };

            assert_eq!(
                infer_types(&create_module(
                    types::Unknown::new(SourceInformation::dummy()).into()
                )),
                Ok(create_module(
                    types::Union::new(
                        vec![
                            types::Number::new(SourceInformation::dummy()).into(),
                            types::None::new(SourceInformation::dummy()).into()
                        ],
                        SourceInformation::dummy()
                    )
                    .into()
                ))
            );
        }

        #[test]
        fn fail_to_infer_case_expressions_with_wrong_argument() {
            assert_debug_snapshot!(infer_types(&Module::from_definitions(vec![
                VariableDefinition::new(
                    "x",
                    Boolean::new(true, SourceInformation::dummy()),
                    types::Union::new(
                        vec![
                            types::Boolean::new(SourceInformation::dummy()).into(),
                            types::None::new(SourceInformation::dummy()).into()
                        ],
                        SourceInformation::dummy()
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
                                types::Number::new(SourceInformation::dummy()),
                                None::new(SourceInformation::dummy()),
                            ),
                            Alternative::new(
                                types::None::new(SourceInformation::dummy()),
                                None::new(SourceInformation::dummy()),
                            )
                        ],
                        SourceInformation::dummy()
                    ),
                    types::None::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()
            ])));
        }

        #[test]
        fn fail_to_infer_type_with_argument_type_of_neither_union_nor_any() {
            assert_debug_snapshot!(infer_types(&Module::from_definitions(vec![
                VariableDefinition::new(
                    "x",
                    Case::new(
                        "y",
                        None::new(SourceInformation::dummy()),
                        vec![Alternative::new(
                            types::Any::new(SourceInformation::dummy()),
                            None::new(SourceInformation::dummy()),
                        )],
                        SourceInformation::dummy()
                    ),
                    types::None::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()
            ])));
        }

        #[test]
        fn fail_to_infer_type_with_alternatives_not_exhaustive_with_union() {
            assert_debug_snapshot!(infer_types(&Module::from_definitions(vec![
                VariableDefinition::new(
                    "x",
                    None::new(SourceInformation::dummy()),
                    types::Union::new(
                        vec![
                            types::Number::new(SourceInformation::dummy()).into(),
                            types::None::new(SourceInformation::dummy()).into(),
                        ],
                        SourceInformation::dummy()
                    ),
                    SourceInformation::dummy(),
                )
                .into(),
                VariableDefinition::new(
                    "y",
                    Case::new(
                        "x",
                        Variable::new("x", SourceInformation::dummy()),
                        vec![Alternative::new(
                            types::Number::new(SourceInformation::dummy()),
                            None::new(SourceInformation::dummy()),
                        )],
                        SourceInformation::dummy()
                    ),
                    types::None::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()
            ])));
        }

        #[test]
        fn fail_to_infer_type_with_alternatives_not_exhaustive_with_any() {
            assert_debug_snapshot!(infer_types(&Module::from_definitions(vec![
                VariableDefinition::new(
                    "x",
                    None::new(SourceInformation::dummy()),
                    types::Any::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into(),
                VariableDefinition::new(
                    "y",
                    Case::new(
                        "x",
                        Variable::new("x", SourceInformation::dummy()),
                        vec![Alternative::new(
                            types::Number::new(SourceInformation::dummy()),
                            None::new(SourceInformation::dummy()),
                        )],
                        SourceInformation::dummy()
                    ),
                    types::None::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()
            ])));
        }

        #[test]
        fn fail_to_infer_type_of_case_expression_with_non_canonical_argument_type_inferred_from_if_expression(
        ) {
            assert_debug_snapshot!(infer_types(&Module::from_definitions(vec![
                VariableDefinition::new(
                    "x",
                    Case::new(
                        "y",
                        If::new(
                            Boolean::new(true, SourceInformation::dummy()),
                            None::new(SourceInformation::dummy()),
                            None::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        ),
                        vec![Alternative::new(
                            types::Number::new(SourceInformation::dummy()),
                            None::new(SourceInformation::dummy()),
                        )],
                        SourceInformation::dummy()
                    ),
                    types::None::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()
            ])));
        }
    }

    mod list_case {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn infer_types_of_list_case_expressions() {
            let create_module = |type_: Type| {
                Module::from_definitions(vec![VariableDefinition::new(
                    "x",
                    ListCase::new(
                        List::with_type(type_.clone(), vec![], SourceInformation::dummy()),
                        type_,
                        "x",
                        "xs",
                        None::new(SourceInformation::dummy()),
                        Variable::new("x", SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    types::None::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()])
            };

            assert_eq!(
                infer_types(&create_module(
                    types::Unknown::new(SourceInformation::dummy()).into()
                )),
                Ok(create_module(
                    types::List::new(
                        types::None::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    )
                    .into()
                ))
            );
        }
    }

    mod record {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn infer_types_of_empty_records() {
            let record_type =
                types::Record::new("Foo", Default::default(), SourceInformation::dummy());
            let reference_type = types::Reference::new("Foo", SourceInformation::dummy());

            assert_debug_snapshot!(infer_types(&Module::from_definitions_and_type_definitions(
                vec![TypeDefinition::new("Foo", record_type)],
                vec![VariableDefinition::new(
                    "x",
                    RecordConstruction::new(
                        reference_type.clone(),
                        Default::default(),
                        SourceInformation::dummy(),
                    ),
                    reference_type,
                    SourceInformation::dummy(),
                )
                .into()],
            )));
        }

        #[test]
        fn infer_types_of_records_with_single_keys() {
            let record_type = types::Record::new(
                "Foo",
                vec![(
                    "foo".into(),
                    types::Number::new(SourceInformation::dummy()).into(),
                )]
                .into_iter()
                .collect(),
                SourceInformation::dummy(),
            );
            let reference_type = types::Reference::new("Foo", SourceInformation::dummy());

            assert_debug_snapshot!(infer_types(&Module::from_definitions_and_type_definitions(
                vec![TypeDefinition::new("Foo", record_type)],
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
            )));
        }

        #[test]
        fn fail_to_infer_types_of_records_due_to_wrong_number_of_keys() {
            let record_type =
                types::Record::new("Foo", Default::default(), SourceInformation::dummy());
            let reference_type = types::Reference::new("Foo", SourceInformation::dummy());

            let module = Module::from_definitions_and_type_definitions(
                vec![TypeDefinition::new("Foo", record_type)],
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
            );
            assert_eq!(
                infer_types(&module),
                Err(CompileError::TypesNotMatched(
                    SourceInformation::dummy().into(),
                    SourceInformation::dummy().into()
                ))
            );
        }

        #[test]
        fn fail_to_infer_types_of_records_due_to_wrong_member_types() {
            let record_type = types::Record::new(
                "Foo",
                vec![(
                    "foo".into(),
                    types::None::new(SourceInformation::dummy()).into(),
                )]
                .into_iter()
                .collect(),
                SourceInformation::dummy(),
            );
            let reference_type = types::Reference::new("Foo", SourceInformation::dummy());

            let module = Module::from_definitions_and_type_definitions(
                vec![TypeDefinition::new("Foo", record_type)],
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
            );
            assert_eq!(
                infer_types(&module),
                Err(CompileError::TypesNotMatched(
                    SourceInformation::dummy().into(),
                    SourceInformation::dummy().into()
                ))
            );
        }

        #[test]
        fn fail_to_infer_types_of_records_due_to_unmatched_identity() {
            let foo_type =
                types::Record::new("Foo", Default::default(), SourceInformation::dummy());
            let bar_type =
                types::Record::new("Bar", Default::default(), SourceInformation::dummy());

            let module = Module::from_definitions_and_type_definitions(
                vec![
                    TypeDefinition::new("Foo", foo_type),
                    TypeDefinition::new("Bar", bar_type),
                ],
                vec![VariableDefinition::new(
                    "x",
                    RecordConstruction::new(
                        types::Reference::new("Foo", SourceInformation::dummy()),
                        vec![(
                            "foo".into(),
                            Number::new(42.0, SourceInformation::dummy()).into(),
                        )]
                        .into_iter()
                        .collect(),
                        SourceInformation::dummy(),
                    ),
                    types::Reference::new("Bar", SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()],
            );

            assert_eq!(
                infer_types(&module),
                Err(CompileError::TypesNotMatched(
                    SourceInformation::dummy().into(),
                    SourceInformation::dummy().into()
                ))
            );
        }

        #[test]
        fn infer_types_of_record_element_operation() {
            let record_type = types::Record::new(
                "Foo",
                vec![(
                    "foo".into(),
                    types::None::new(SourceInformation::dummy()).into(),
                )]
                .into_iter()
                .collect(),
                SourceInformation::dummy(),
            );

            assert_debug_snapshot!(infer_types(&Module::from_definitions_and_type_definitions(
                vec![TypeDefinition::new("Foo", record_type)],
                vec![VariableDefinition::new(
                    "x",
                    RecordElementOperation::new(
                        types::Reference::new("Foo", SourceInformation::dummy()),
                        "foo",
                        RecordConstruction::new(
                            types::Reference::new("Foo", SourceInformation::dummy()),
                            vec![("foo".into(), None::new(SourceInformation::dummy()).into())]
                                .into_iter()
                                .collect(),
                            SourceInformation::dummy(),
                        ),
                        SourceInformation::dummy(),
                    ),
                    types::None::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()],
            )));
        }
    }

    mod operation {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn infer_types_of_arithmetic_operations() {
            let module = Module::from_definitions(vec![VariableDefinition::new(
                "x",
                ArithmeticOperation::new(
                    ArithmeticOperator::Add,
                    Number::new(42.0, SourceInformation::dummy()),
                    Number::new(42.0, SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()]);

            assert_eq!(infer_types(&module), Ok(module.clone()));
        }

        #[test]
        fn infer_types_of_number_comparison_operations() {
            let module = Module::from_definitions(vec![VariableDefinition::new(
                "x",
                OrderOperation::new(
                    OrderOperator::LessThan,
                    Number::new(42.0, SourceInformation::dummy()),
                    Number::new(42.0, SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                types::Boolean::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()]);

            assert_eq!(infer_types(&module), Ok(module.clone()));
        }

        #[test]
        fn infer_types_of_equal_operation() {
            let record_type =
                types::Record::new("Foo", Default::default(), SourceInformation::dummy());
            let reference_type = types::Reference::new("Foo", SourceInformation::dummy());

            assert_debug_snapshot!(infer_types(&Module::from_definitions_and_type_definitions(
                vec![TypeDefinition::new("Foo", record_type)],
                vec![VariableDefinition::new(
                    "x",
                    EqualityOperation::with_type(
                        types::Unknown::new(SourceInformation::dummy()),
                        EqualityOperator::Equal,
                        RecordConstruction::new(
                            reference_type.clone(),
                            Default::default(),
                            SourceInformation::dummy(),
                        ),
                        RecordConstruction::new(
                            reference_type,
                            Default::default(),
                            SourceInformation::dummy(),
                        ),
                        SourceInformation::dummy(),
                    ),
                    types::Boolean::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()],
            )));
        }

        #[test]
        fn infer_types_of_boolean_operation() {
            let module = Module::from_definitions(vec![VariableDefinition::new(
                "x",
                BooleanOperation::new(
                    BooleanOperator::And,
                    Boolean::new(true, SourceInformation::dummy()),
                    Boolean::new(true, SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                types::Boolean::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()]);

            assert_eq!(infer_types(&module), Ok(module.clone()));
        }

        #[test]
        fn infer_types_of_pipe_operation() {
            let module = Module::from_definitions(vec![
                FunctionDefinition::new(
                    "f",
                    vec!["x".into()],
                    Number::new(42.0, SourceInformation::dummy()),
                    types::Function::new(
                        types::None::new(SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    SourceInformation::dummy(),
                )
                .into(),
                VariableDefinition::new(
                    "x",
                    PipeOperation::new(
                        None::new(SourceInformation::dummy()),
                        Variable::new("f", SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into(),
            ]);

            assert_eq!(infer_types(&module), Ok(module.clone()));
        }
    }

    mod union {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn infer_casting_of_non_union_types_to_union_types() {
            let module = Module::from_definitions(vec![VariableDefinition::new(
                "x",
                Boolean::new(true, SourceInformation::dummy()),
                types::Union::new(
                    vec![
                        types::Boolean::new(SourceInformation::dummy()).into(),
                        types::None::new(SourceInformation::dummy()).into(),
                    ],
                    SourceInformation::dummy(),
                ),
                SourceInformation::dummy(),
            )
            .into()]);

            assert_eq!(infer_types(&module), Ok(module));
        }

        #[test]
        fn infer_casting_of_lower_union_types_to_upper_union_types() {
            let module = Module::from_definitions(vec![
                VariableDefinition::new(
                    "x",
                    Boolean::new(true, SourceInformation::dummy()),
                    types::Union::new(
                        vec![
                            types::Boolean::new(SourceInformation::dummy()).into(),
                            types::None::new(SourceInformation::dummy()).into(),
                        ],
                        SourceInformation::dummy(),
                    ),
                    SourceInformation::dummy(),
                )
                .into(),
                VariableDefinition::new(
                    "y",
                    Variable::new("x", SourceInformation::dummy()),
                    types::Union::new(
                        vec![
                            types::Boolean::new(SourceInformation::dummy()).into(),
                            types::None::new(SourceInformation::dummy()).into(),
                            types::Number::new(SourceInformation::dummy()).into(),
                        ],
                        SourceInformation::dummy(),
                    ),
                    SourceInformation::dummy(),
                )
                .into(),
            ]);

            assert_eq!(infer_types(&module), Ok(module));
        }

        #[test]
        fn fail_to_infer_casting_of_upper_union_types_to_lower_union_types() {
            let module = Module::from_definitions(vec![
                VariableDefinition::new(
                    "x",
                    Boolean::new(true, SourceInformation::dummy()),
                    types::Union::new(
                        vec![
                            types::Boolean::new(SourceInformation::dummy()).into(),
                            types::None::new(SourceInformation::dummy()).into(),
                            types::Number::new(SourceInformation::dummy()).into(),
                        ],
                        SourceInformation::dummy(),
                    ),
                    SourceInformation::dummy(),
                )
                .into(),
                VariableDefinition::new(
                    "y",
                    Variable::new("x", SourceInformation::dummy()),
                    types::Union::new(
                        vec![
                            types::Boolean::new(SourceInformation::dummy()).into(),
                            types::None::new(SourceInformation::dummy()).into(),
                        ],
                        SourceInformation::dummy(),
                    ),
                    SourceInformation::dummy(),
                )
                .into(),
            ]);

            assert_eq!(
                infer_types(&module),
                Err(CompileError::TypesNotMatched(
                    SourceInformation::dummy().into(),
                    SourceInformation::dummy().into()
                ))
            );
        }

        #[test]
        fn infer_contravariance_of_function_types() {
            let module = Module::from_definitions(vec![
                FunctionDefinition::new(
                    "f",
                    vec!["x".into()],
                    Boolean::new(true, SourceInformation::dummy()),
                    types::Function::new(
                        types::Union::new(
                            vec![
                                types::Boolean::new(SourceInformation::dummy()).into(),
                                types::None::new(SourceInformation::dummy()).into(),
                            ],
                            SourceInformation::dummy(),
                        ),
                        types::Boolean::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    SourceInformation::dummy(),
                )
                .into(),
                VariableDefinition::new(
                    "g",
                    Variable::new("f", SourceInformation::dummy()),
                    types::Function::new(
                        types::Boolean::new(SourceInformation::dummy()),
                        types::Union::new(
                            vec![
                                types::Boolean::new(SourceInformation::dummy()).into(),
                                types::None::new(SourceInformation::dummy()).into(),
                            ],
                            SourceInformation::dummy(),
                        ),
                        SourceInformation::dummy(),
                    ),
                    SourceInformation::dummy(),
                )
                .into(),
            ]);

            assert_eq!(infer_types(&module), Ok(module));
        }

        #[test]
        fn canonicalize_record_type_in_type_definitions() {
            let module = Module::from_definitions_and_type_definitions(
                vec![TypeDefinition::new(
                    "x",
                    types::Record::new(
                        "Foo",
                        vec![(
                            "foo".into(),
                            types::Union::new(
                                vec![
                                    types::Any::new(SourceInformation::dummy()).into(),
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
                )],
                vec![],
            );

            assert_debug_snapshot!(infer_types(&module));
        }
    }

    mod any {
        use super::*;

        #[test]
        fn infer_casting_of_non_union_types_to_any_types() {
            assert_debug_snapshot!(infer_types(&Module::from_definitions(vec![
                VariableDefinition::new(
                    "x",
                    Boolean::new(true, SourceInformation::dummy()),
                    types::Any::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()
            ])));
        }

        #[test]
        fn infer_casting_of_union_types_to_any_types() {
            assert_debug_snapshot!(infer_types(&Module::from_definitions(vec![
                VariableDefinition::new(
                    "x",
                    Boolean::new(true, SourceInformation::dummy()),
                    types::Union::new(
                        vec![
                            types::Boolean::new(SourceInformation::dummy()).into(),
                            types::None::new(SourceInformation::dummy()).into(),
                        ],
                        SourceInformation::dummy(),
                    ),
                    SourceInformation::dummy(),
                )
                .into(),
                VariableDefinition::new(
                    "y",
                    Variable::new("x", SourceInformation::dummy()),
                    types::Any::new(SourceInformation::dummy(),),
                    SourceInformation::dummy(),
                )
                .into()
            ])));
        }

        #[test]
        fn infer_any_types_from_any_types() {
            assert_debug_snapshot!(infer_types(&Module::from_definitions(vec![
                VariableDefinition::new(
                    "x",
                    Number::new(42.0, SourceInformation::dummy()),
                    types::Any::new(SourceInformation::dummy(),),
                    SourceInformation::dummy(),
                )
                .into(),
                VariableDefinition::new(
                    "y",
                    Let::new(
                        vec![VariableDefinition::new(
                            "z",
                            Variable::new("x", SourceInformation::dummy()),
                            types::Unknown::new(SourceInformation::dummy()),
                            SourceInformation::dummy(),
                        )],
                        Number::new(42.0, SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()
            ])));
        }

        #[test]
        fn infer_any_types_from_union_of_concrete_type_and_any_type() {
            assert_debug_snapshot!(infer_types(&Module::from_definitions(vec![
                VariableDefinition::new(
                    "x",
                    Number::new(42.0, SourceInformation::dummy()),
                    types::Any::new(SourceInformation::dummy(),),
                    SourceInformation::dummy(),
                )
                .into(),
                VariableDefinition::new(
                    "y",
                    Let::new(
                        vec![VariableDefinition::new(
                            "z",
                            If::new(
                                Boolean::new(true, SourceInformation::dummy()),
                                Variable::new("x", SourceInformation::dummy()),
                                Number::new(42.0, SourceInformation::dummy()),
                                SourceInformation::dummy()
                            ),
                            types::Unknown::new(SourceInformation::dummy()),
                            SourceInformation::dummy(),
                        )],
                        Number::new(42.0, SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()
            ])));
        }
    }

    mod lists {
        use super::*;

        #[test]
        fn infer_type_of_empty_list_literal() {
            let list_type = types::List::new(
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            );

            assert_debug_snapshot!(infer_types(&Module::from_definitions(vec![
                VariableDefinition::new(
                    "x",
                    List::new(vec![], SourceInformation::dummy()),
                    list_type,
                    SourceInformation::dummy(),
                )
                .into()
            ])));
        }

        #[test]
        fn infer_type_of_list_with_an_element() {
            let list_type = types::List::new(
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            );

            assert_debug_snapshot!(infer_types(&Module::from_definitions(vec![
                VariableDefinition::new(
                    "x",
                    List::new(
                        vec![ListElement::Single(
                            Number::new(42.0, SourceInformation::dummy()).into()
                        )],
                        SourceInformation::dummy(),
                    ),
                    list_type,
                    SourceInformation::dummy(),
                )
                .into()
            ])));
        }

        #[test]
        fn infer_type_of_list_with_spread_element() {
            let list_type = types::List::new(
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            );

            assert_debug_snapshot!(infer_types(&Module::from_definitions(vec![
                VariableDefinition::new(
                    "x",
                    List::new(
                        vec![ListElement::Multiple(
                            List::new(vec![], SourceInformation::dummy(),).into()
                        )],
                        SourceInformation::dummy(),
                    ),
                    list_type,
                    SourceInformation::dummy(),
                )
                .into()
            ])));
        }

        #[test]
        #[should_panic]
        fn do_not_allow_covariance_with_list() {
            infer_types(&Module::from_definitions(vec![
                VariableDefinition::new(
                    "x",
                    List::new(vec![], SourceInformation::dummy()),
                    types::List::new(
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    SourceInformation::dummy(),
                )
                .into(),
                VariableDefinition::new(
                    "y",
                    Variable::new("x", SourceInformation::dummy()),
                    types::List::new(
                        types::Any::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    SourceInformation::dummy(),
                )
                .into(),
            ]))
            .unwrap();
        }

        #[test]
        fn allow_covariance_with_list() {
            infer_types(&Module::from_definitions(vec![
                VariableDefinition::new(
                    "x",
                    List::new(vec![], SourceInformation::dummy()),
                    types::List::new(
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    SourceInformation::dummy(),
                )
                .into(),
                VariableDefinition::new(
                    "y",
                    List::new(
                        vec![ListElement::Multiple(
                            Variable::new("x", SourceInformation::dummy()).into(),
                        )],
                        SourceInformation::dummy(),
                    ),
                    types::List::new(
                        types::Any::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    SourceInformation::dummy(),
                )
                .into(),
            ]))
            .unwrap();
        }
    }
}
