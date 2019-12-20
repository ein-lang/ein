mod equation;
mod error;
mod type_inferrer;

use crate::ast::*;
pub use error::*;
use type_inferrer::*;

pub fn infer_types(module: &Module) -> Result<Module, TypeInferenceError> {
    TypeInferrer::new().infer(module)
}

#[cfg(test)]
mod tests {
    use super::error::*;
    use super::infer_types;
    use crate::ast::*;
    use crate::debug::*;
    use crate::package::Package;
    use crate::path::*;
    use crate::types;
    use pretty_assertions::assert_eq;

    #[test]
    fn infer_types_with_empty_modules() {
        assert_eq!(
            infer_types(&Module::from_definitions(vec![])),
            Ok(Module::from_definitions(vec![]))
        );
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
            Err(TypeInferenceError::TypesNotMatched(
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
            Err(TypeInferenceError::TypesNotMatched(
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
            Err(TypeInferenceError::TypesNotMatched(
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
            Err(TypeInferenceError::TypesNotMatched(
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
            Err(TypeInferenceError::TypesNotMatched(
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
                        types::Variable::new(SourceInformation::dummy()),
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
            Err(TypeInferenceError::VariableNotFound(
                "y".into(),
                SourceInformation::dummy().into()
            ))
        );
    }

    #[test]
    fn infer_types_of_variables_with_imported_modules() {
        let module = Module::new(
            ModulePath::new(Package::new("", ""), vec![]),
            Export::new(Default::default()),
            vec![ModuleInterface::new(
                ModulePath::new(Package::new("m", ""), vec![]),
                vec![(
                    "x".into(),
                    types::Number::new(SourceInformation::dummy()).into(),
                )]
                .into_iter()
                .collect(),
            )],
            vec![],
            vec![ValueDefinition::new(
                "y",
                Variable::new("m.x", SourceInformation::dummy()),
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
                            types::Variable::new(SourceInformation::dummy()),
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
                            types::Variable::new(SourceInformation::dummy()),
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
                            types::Variable::new(SourceInformation::dummy()),
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
            Err(TypeInferenceError::VariableNotFound(
                "b".into(),
                SourceInformation::dummy().into(),
            ))
        );
    }
}
