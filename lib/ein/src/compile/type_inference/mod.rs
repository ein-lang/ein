mod equation;
mod equation_set;
mod error;
mod type_inferrer;

use crate::ast::*;
use crate::types::{self, Type};
pub use error::*;
use type_inferrer::*;

pub fn infer_types(module: &Module) -> Result<Module, TypeInferenceError> {
    TypeInferrer::new().infer(&module.convert_types(&mut |type_| match type_ {
        Type::Unknown(unknown) => types::Variable::new(unknown.source_information().clone()).into(),
        _ => type_.clone(),
    }))
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
                Default::default(),
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
            Err(TypeInferenceError::VariableNotFound(
                "b".into(),
                SourceInformation::dummy().into(),
            ))
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
        assert_eq!(infer_types(&module), Ok(module));
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
            Err(TypeInferenceError::TypeNotFound {
                reference: types::Reference::new("Foo", SourceInformation::dummy())
            })
        );
    }

    #[test]
    fn infer_types_with_imported_reference_types() {
        let module = Module::new(
            ModulePath::new(Package::new("", ""), vec![]),
            Export::new(Default::default()),
            vec![ModuleInterface::new(
                ModulePath::new(Package::new("Module", ""), vec![]),
                vec![(
                    "Foo".into(),
                    types::Number::new(SourceInformation::dummy()).into(),
                )]
                .into_iter()
                .collect(),
                Default::default(),
            )],
            vec![],
            vec![ValueDefinition::new(
                "x",
                Number::new(42.0, SourceInformation::dummy()),
                types::Reference::new("Module.Foo", SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()],
        );
        assert_eq!(infer_types(&module), Ok(module));
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
        assert_eq!(infer_types(&module), Ok(module));
    }

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
            Err(TypeInferenceError::TypesNotMatched(
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
            Err(TypeInferenceError::TypesNotMatched(
                SourceInformation::dummy().into(),
                SourceInformation::dummy().into()
            ))
        );
    }
}
