use super::super::error::CompileError;
use super::super::reference_type_resolver::ReferenceTypeResolver;
use super::super::type_equality_checker::TypeEqualityChecker;
use super::super::union_type_simplifier::UnionTypeSimplifier;
use super::constraint_checker::ConstraintChecker;
use super::constraint_collector::ConstraintCollector;
use super::constraint_solver::ConstraintSolver;
use super::variable_substitutor::VariableSubstitutor;
use crate::ast::*;
use crate::types::{self, Type};
use std::sync::Arc;

pub struct TypeInferrer {
    reference_type_resolver: Arc<ReferenceTypeResolver>,
    type_equality_checker: Arc<TypeEqualityChecker>,
    union_type_simplifier: Arc<UnionTypeSimplifier>,
}

impl TypeInferrer {
    pub fn new(
        reference_type_resolver: Arc<ReferenceTypeResolver>,
        type_equality_checker: Arc<TypeEqualityChecker>,
        union_type_simplifier: Arc<UnionTypeSimplifier>,
    ) -> Self {
        Self {
            reference_type_resolver,
            type_equality_checker,
            union_type_simplifier,
        }
    }

    pub fn infer(&self, module: &Module) -> Result<Module, CompileError> {
        let module = module.convert_types(&mut |type_| -> Result<_, CompileError> {
            Ok(match type_ {
                Type::Unknown(unknown) => {
                    types::Variable::new(unknown.source_information().clone()).into()
                }
                _ => type_.clone(),
            })
        })?;

        let (solved_subsumption_set, mut checked_subsumption_set) =
            ConstraintCollector::new(self.reference_type_resolver.clone()).collect(&module)?;

        let substitutions = ConstraintSolver::new(self.reference_type_resolver.clone())
            .solve(solved_subsumption_set, &mut checked_subsumption_set)?;

        let substitutor =
            VariableSubstitutor::new(self.union_type_simplifier.clone(), substitutions);

        let checker = ConstraintChecker::new(
            substitutor.clone(),
            self.reference_type_resolver.clone(),
            self.type_equality_checker.clone(),
        );

        checker.check(checked_subsumption_set)?;

        module.convert_types(&mut |type_| -> Result<_, CompileError> {
            substitutor.substitute(type_)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug::*;
    use crate::package::Package;
    use crate::path::*;
    use crate::types::{self, Type};
    use insta::assert_debug_snapshot;
    use pretty_assertions::assert_eq;

    fn infer_types(module: &Module) -> Result<Module, CompileError> {
        let reference_type_resolver = ReferenceTypeResolver::new(&module);
        let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());
        let union_type_simplifier = UnionTypeSimplifier::new(
            reference_type_resolver.clone(),
            type_equality_checker.clone(),
        );

        TypeInferrer::new(
            reference_type_resolver,
            type_equality_checker,
            union_type_simplifier,
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
        let module = Module::from_definitions(vec![ValueDefinition::new(
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
        let module = Module::from_definitions(vec![ValueDefinition::new(
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
        let module = Module::from_definitions(vec![ValueDefinition::new(
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
            ValueDefinition::new(
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
            ValueDefinition::new(
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
    fn infer_types_of_let_values() {
        let module = Module::from_definitions(vec![ValueDefinition::new(
            "x",
            Let::new(
                vec![ValueDefinition::new(
                    "y",
                    Number::new(42.0, SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()],
                Variable::new("y", SourceInformation::dummy()),
            ),
            types::Number::new(SourceInformation::dummy()),
            SourceInformation::dummy(),
        )
        .into()]);

        assert_eq!(infer_types(&module), Ok(module));
    }

    #[test]
    fn fail_to_infer_types_of_let_values() {
        let module = Module::from_definitions(vec![ValueDefinition::new(
            "x",
            Let::new(
                vec![ValueDefinition::new(
                    "y",
                    Number::new(42.0, SourceInformation::dummy()),
                    types::Function::new(
                        types::Number::new(SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    SourceInformation::dummy(),
                )
                .into()],
                Variable::new("y", SourceInformation::dummy()),
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
    fn infer_types_of_let_functions() {
        let module = Module::from_definitions(vec![ValueDefinition::new(
            "x",
            Let::new(
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
                )
                .into()],
                Application::new(
                    Variable::new("f", SourceInformation::dummy()),
                    Number::new(42.0, SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
            ),
            types::Number::new(SourceInformation::dummy()),
            SourceInformation::dummy(),
        )
        .into()]);

        assert_eq!(infer_types(&module), Ok(module));
    }

    #[test]
    fn fail_to_infer_types_of_let_functions() {
        let module = Module::from_definitions(vec![ValueDefinition::new(
            "x",
            Let::new(
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
                )
                .into()],
                Application::new(
                    Variable::new("f", SourceInformation::dummy()),
                    Number::new(42.0, SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
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
    fn infer_types_of_let_values_with_type_variables() {
        assert_eq!(
            infer_types(&Module::from_definitions(vec![ValueDefinition::new(
                "x",
                Let::new(
                    vec![ValueDefinition::new(
                        "y",
                        Number::new(42.0, SourceInformation::dummy()),
                        types::Unknown::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )
                    .into()],
                    Variable::new("y", SourceInformation::dummy()),
                ),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()])),
            Ok(Module::from_definitions(vec![ValueDefinition::new(
                "x",
                Let::new(
                    vec![ValueDefinition::new(
                        "y",
                        Number::new(42.0, SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )
                    .into(),],
                    Variable::new("y", SourceInformation::dummy()),
                ),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()]))
        );
    }

    #[test]
    fn fail_to_infer_types_with_missing_variables() {
        let module = Module::from_definitions(vec![ValueDefinition::new(
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
            vec![ValueDefinition::new(
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
    fn infer_types_of_let_values_with_recursive_functions_and_the_latter_typed() {
        assert_eq!(
            infer_types(&Module::from_definitions(vec![ValueDefinition::new(
                "x",
                Let::new(
                    vec![
                        ValueDefinition::new(
                            "f",
                            Variable::new("g", SourceInformation::dummy()),
                            types::Unknown::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        )
                        .into(),
                        ValueDefinition::new(
                            "g",
                            Variable::new("f", SourceInformation::dummy()),
                            types::Function::new(
                                types::Number::new(SourceInformation::dummy()),
                                types::Number::new(SourceInformation::dummy()),
                                SourceInformation::dummy(),
                            ),
                            SourceInformation::dummy()
                        )
                        .into()
                    ],
                    Application::new(
                        Variable::new("f", SourceInformation::dummy()),
                        Number::new(42.0, SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                ),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()])),
            Ok(Module::from_definitions(vec![ValueDefinition::new(
                "x",
                Let::new(
                    vec![
                        ValueDefinition::new(
                            "f",
                            Variable::new("g", SourceInformation::dummy()),
                            types::Function::new(
                                types::Number::new(SourceInformation::dummy()),
                                types::Number::new(SourceInformation::dummy()),
                                SourceInformation::dummy(),
                            ),
                            SourceInformation::dummy()
                        )
                        .into(),
                        ValueDefinition::new(
                            "g",
                            Variable::new("f", SourceInformation::dummy()),
                            types::Function::new(
                                types::Number::new(SourceInformation::dummy()),
                                types::Number::new(SourceInformation::dummy()),
                                SourceInformation::dummy(),
                            ),
                            SourceInformation::dummy()
                        )
                        .into()
                    ],
                    Application::new(
                        Variable::new("f", SourceInformation::dummy()),
                        Number::new(42.0, SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                ),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()]))
        );
    }

    #[test]
    fn infer_types_of_let_values_with_recursive_functions_and_the_former_typed() {
        assert_eq!(
            infer_types(&Module::from_definitions(vec![ValueDefinition::new(
                "x",
                Let::new(
                    vec![
                        ValueDefinition::new(
                            "f",
                            Variable::new("g", SourceInformation::dummy()),
                            types::Function::new(
                                types::Number::new(SourceInformation::dummy()),
                                types::Number::new(SourceInformation::dummy()),
                                SourceInformation::dummy(),
                            ),
                            SourceInformation::dummy()
                        )
                        .into(),
                        ValueDefinition::new(
                            "g",
                            Variable::new("f", SourceInformation::dummy()),
                            types::Unknown::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        )
                        .into()
                    ],
                    Application::new(
                        Variable::new("f", SourceInformation::dummy()),
                        Number::new(42.0, SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                ),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()])),
            Ok(Module::from_definitions(vec![ValueDefinition::new(
                "x",
                Let::new(
                    vec![
                        ValueDefinition::new(
                            "f",
                            Variable::new("g", SourceInformation::dummy()),
                            types::Function::new(
                                types::Number::new(SourceInformation::dummy()),
                                types::Number::new(SourceInformation::dummy()),
                                SourceInformation::dummy(),
                            ),
                            SourceInformation::dummy()
                        )
                        .into(),
                        ValueDefinition::new(
                            "g",
                            Variable::new("f", SourceInformation::dummy()),
                            types::Function::new(
                                types::Number::new(SourceInformation::dummy()),
                                types::Number::new(SourceInformation::dummy()),
                                SourceInformation::dummy(),
                            ),
                            SourceInformation::dummy()
                        )
                        .into()
                    ],
                    Application::new(
                        Variable::new("f", SourceInformation::dummy()),
                        Number::new(42.0, SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                ),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()]))
        );
    }

    #[test]
    fn infer_types_of_let_values_in_function_definition_with_recursive_functions() {
        assert_eq!(
            infer_types(&Module::from_definitions(vec![FunctionDefinition::new(
                "f",
                vec!["x".into()],
                Let::new(
                    vec![
                        ValueDefinition::new(
                            "f",
                            Variable::new("g", SourceInformation::dummy()),
                            types::Unknown::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        )
                        .into(),
                        ValueDefinition::new(
                            "g",
                            Variable::new("f", SourceInformation::dummy()),
                            types::Function::new(
                                types::Number::new(SourceInformation::dummy()),
                                types::Number::new(SourceInformation::dummy()),
                                SourceInformation::dummy(),
                            ),
                            SourceInformation::dummy()
                        )
                        .into()
                    ],
                    Application::new(
                        Variable::new("f", SourceInformation::dummy()),
                        Number::new(42.0, SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                ),
                types::Function::new(
                    types::Number::new(SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                SourceInformation::dummy(),
            )
            .into()])),
            Ok(Module::from_definitions(vec![FunctionDefinition::new(
                "f",
                vec!["x".into()],
                Let::new(
                    vec![
                        ValueDefinition::new(
                            "f",
                            Variable::new("g", SourceInformation::dummy()),
                            types::Function::new(
                                types::Number::new(SourceInformation::dummy()),
                                types::Number::new(SourceInformation::dummy()),
                                SourceInformation::dummy(),
                            ),
                            SourceInformation::dummy()
                        )
                        .into(),
                        ValueDefinition::new(
                            "g",
                            Variable::new("f", SourceInformation::dummy()),
                            types::Function::new(
                                types::Number::new(SourceInformation::dummy()),
                                types::Number::new(SourceInformation::dummy()),
                                SourceInformation::dummy(),
                            ),
                            SourceInformation::dummy()
                        )
                        .into()
                    ],
                    Application::new(
                        Variable::new("f", SourceInformation::dummy()),
                        Number::new(42.0, SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                ),
                types::Function::new(
                    types::Number::new(SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                SourceInformation::dummy(),
            )
            .into()]))
        );
    }

    #[test]
    fn fail_to_infer_types_of_recursive_value_definitions() {
        let module = Module::from_definitions(vec![ValueDefinition::new(
            "x",
            Let::new(
                vec![
                    ValueDefinition::new(
                        "a",
                        Variable::new("b", SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    )
                    .into(),
                    ValueDefinition::new(
                        "b",
                        Variable::new("a", SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    )
                    .into(),
                ],
                Variable::new("a", SourceInformation::dummy()),
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
        let module = Module::new(
            ModulePath::new(Package::new("", ""), vec![]),
            Export::new(Default::default()),
            vec![],
            vec![TypeDefinition::new(
                "Foo",
                types::Number::new(SourceInformation::dummy()),
            )],
            vec![ValueDefinition::new(
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
        let module = Module::new(
            ModulePath::new(Package::new("", ""), vec![]),
            Export::new(Default::default()),
            vec![],
            vec![],
            vec![ValueDefinition::new(
                "x",
                Number::new(42.0, SourceInformation::dummy()),
                types::Reference::new("Foo", SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()],
        );

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
            vec![ValueDefinition::new(
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
        let module = Module::new(
            ModulePath::new(Package::new("", ""), vec![]),
            Export::new(Default::default()),
            vec![],
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
                ValueDefinition::new(
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

    mod if_ {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn infer_types_of_if_expressions() {
            let module = Module::from_definitions(vec![ValueDefinition::new(
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
            let module = Module::from_definitions(vec![ValueDefinition::new(
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
            let module = Module::from_definitions(vec![ValueDefinition::new(
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
                Module::from_definitions(vec![ValueDefinition::new(
                    "x",
                    Let::new(
                        vec![ValueDefinition::new(
                            "y",
                            If::new(
                                Boolean::new(true, SourceInformation::dummy()),
                                Boolean::new(true, SourceInformation::dummy()),
                                None::new(SourceInformation::dummy()),
                                SourceInformation::dummy(),
                            ),
                            type_,
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
                Module::from_definitions(vec![ValueDefinition::new(
                    "x",
                    Let::new(
                        vec![ValueDefinition::new(
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
                Module::from_definitions(vec![ValueDefinition::new(
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
                ValueDefinition::new(
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
                ValueDefinition::new(
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
        fn fail_to_infer_type_of_case_expression_with_non_canonical_argument_type() {
            assert_debug_snapshot!(infer_types(&Module::from_definitions(vec![
                ValueDefinition::new(
                    "x",
                    Case::new(
                        "y",
                        None::new(SourceInformation::dummy()),
                        vec![Alternative::new(
                            types::Any::new(SourceInformation::dummy()),
                            None::new(SourceInformation::dummy()),
                        )
                        .into()],
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
                ValueDefinition::new(
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
                        )
                        .into()],
                        SourceInformation::dummy()
                    ),
                    types::None::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()
            ])));
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
                vec![ValueDefinition::new(
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
                vec![ValueDefinition::new(
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
                vec![ValueDefinition::new(
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
                vec![ValueDefinition::new(
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
                vec![ValueDefinition::new(
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
        fn infer_types_of_record_element_operations() {
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
                vec![TypeDefinition::new("Foo", record_type.clone())],
                vec![ValueDefinition::new(
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
            let create_module = |type_: Type| {
                Module::from_definitions(vec![ValueDefinition::new(
                    "x",
                    Operation::with_type(
                        type_,
                        Operator::Add,
                        Number::new(42.0, SourceInformation::dummy()),
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
                    types::Number::new(SourceInformation::dummy()).into()
                ))
            );
        }

        #[test]
        fn infer_types_of_number_comparison_operations() {
            let create_module = |type_: Type| {
                Module::from_definitions(vec![ValueDefinition::new(
                    "x",
                    Operation::with_type(
                        type_,
                        Operator::LessThan,
                        Number::new(42.0, SourceInformation::dummy()),
                        Number::new(42.0, SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    types::Boolean::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()])
            };

            assert_eq!(
                infer_types(&create_module(
                    types::Unknown::new(SourceInformation::dummy()).into()
                )),
                Ok(create_module(
                    types::Number::new(SourceInformation::dummy()).into()
                ))
            );
        }

        #[test]
        fn infer_types_of_equal_operation() {
            let record_type =
                types::Record::new("Foo", Default::default(), SourceInformation::dummy());
            let reference_type = types::Reference::new("Foo", SourceInformation::dummy());

            assert_debug_snapshot!(infer_types(&Module::from_definitions_and_type_definitions(
                vec![TypeDefinition::new("Foo", record_type.clone())],
                vec![ValueDefinition::new(
                    "x",
                    Operation::with_type(
                        types::Unknown::new(SourceInformation::dummy()),
                        Operator::Equal,
                        RecordConstruction::new(
                            reference_type.clone(),
                            Default::default(),
                            SourceInformation::dummy(),
                        ),
                        RecordConstruction::new(
                            reference_type.clone(),
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
            let create_module = |type_: Type| {
                Module::from_definitions(vec![ValueDefinition::new(
                    "x",
                    Operation::with_type(
                        type_,
                        Operator::And,
                        Boolean::new(true, SourceInformation::dummy()),
                        Boolean::new(true, SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    types::Boolean::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()])
            };

            assert_eq!(
                infer_types(&create_module(
                    types::Unknown::new(SourceInformation::dummy()).into()
                )),
                Ok(create_module(
                    types::Boolean::new(SourceInformation::dummy()).into()
                ))
            );
        }
    }

    mod union {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn infer_casting_of_non_union_types_to_union_types() {
            let module = Module::from_definitions(vec![ValueDefinition::new(
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
                ValueDefinition::new(
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
                ValueDefinition::new(
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
                ValueDefinition::new(
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
                ValueDefinition::new(
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
                ValueDefinition::new(
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
        fn simplify_record_type_in_type_definitions() {
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
                ValueDefinition::new(
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
                ValueDefinition::new(
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
                ValueDefinition::new(
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
                ValueDefinition::new(
                    "x",
                    Number::new(42.0, SourceInformation::dummy()),
                    types::Any::new(SourceInformation::dummy(),),
                    SourceInformation::dummy(),
                )
                .into(),
                ValueDefinition::new(
                    "y",
                    Let::new(
                        vec![ValueDefinition::new(
                            "z",
                            Variable::new("x", SourceInformation::dummy()),
                            types::Unknown::new(SourceInformation::dummy()),
                            SourceInformation::dummy(),
                        )
                        .into()],
                        Number::new(42.0, SourceInformation::dummy()),
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
                ValueDefinition::new(
                    "x",
                    Number::new(42.0, SourceInformation::dummy()),
                    types::Any::new(SourceInformation::dummy(),),
                    SourceInformation::dummy(),
                )
                .into(),
                ValueDefinition::new(
                    "y",
                    Let::new(
                        vec![ValueDefinition::new(
                            "z",
                            If::new(
                                Boolean::new(true, SourceInformation::dummy()),
                                Variable::new("x", SourceInformation::dummy()),
                                Number::new(42.0, SourceInformation::dummy()),
                                SourceInformation::dummy()
                            ),
                            types::Unknown::new(SourceInformation::dummy()),
                            SourceInformation::dummy(),
                        )
                        .into()],
                        Number::new(42.0, SourceInformation::dummy()),
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
                ValueDefinition::new(
                    "x",
                    List::new(vec![], SourceInformation::dummy()),
                    list_type.clone(),
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
                ValueDefinition::new(
                    "x",
                    List::new(
                        vec![ListElement::Single(
                            Number::new(42.0, SourceInformation::dummy()).into()
                        )],
                        SourceInformation::dummy(),
                    ),
                    list_type.clone(),
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
                ValueDefinition::new(
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
    }
}
