use super::super::name_generator::NameGenerator;
use crate::ast::*;
use crate::types;

pub fn desugar_non_variable_applications(module: &Module) -> Module {
    let mut name_generator = NameGenerator::new("generated_function_");

    module.convert_expressions(&mut |expression| match expression {
        Expression::Application(application) => match application.function() {
            Expression::Application(_) | Expression::Variable(_) => application.clone().into(),
            // Treat let expressions in a special way to omit extra let expressions.
            Expression::Let(let_) => Let::new(
                let_.definitions().to_vec(),
                Application::new(
                    let_.expression().clone(),
                    application.argument().clone(),
                    application.source_information().clone(),
                ),
            )
            .into(),
            function => {
                let function_name = name_generator.generate();
                let source_information = application.source_information();

                Let::new(
                    vec![ValueDefinition::new(
                        function_name.clone(),
                        function.clone(),
                        types::Unknown::new(source_information.clone()),
                        source_information.clone(),
                    )
                    .into()],
                    Application::new(
                        Variable::new(function_name, source_information.clone()),
                        application.argument().clone(),
                        source_information.clone(),
                    ),
                )
                .into()
            }
        },
        _ => expression.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug::*;
    use crate::types;
    use pretty_assertions::assert_eq;

    #[test]
    fn convert_non_variable_applications() {
        assert_eq!(
            desugar_non_variable_applications(&Module::from_definitions(vec![
                ValueDefinition::new(
                    "x",
                    Application::new(
                        Let::new(
                            vec![FunctionDefinition::new(
                                "f",
                                vec!["y".into()],
                                Variable::new("y", SourceInformation::dummy()),
                                types::Function::new(
                                    types::Number::new(SourceInformation::dummy()),
                                    types::Number::new(SourceInformation::dummy()),
                                    SourceInformation::dummy()
                                ),
                                SourceInformation::dummy(),
                            )
                            .into()],
                            Variable::new("f", SourceInformation::dummy())
                        ),
                        Variable::new("z", SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()
            ])),
            Module::from_definitions(vec![ValueDefinition::new(
                "x",
                Let::new(
                    vec![FunctionDefinition::new(
                        "f",
                        vec!["y".into()],
                        Variable::new("y", SourceInformation::dummy()),
                        types::Function::new(
                            types::Number::new(SourceInformation::dummy()),
                            types::Number::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        ),
                        SourceInformation::dummy(),
                    )
                    .into()],
                    Application::new(
                        Variable::new("f", SourceInformation::dummy()),
                        Variable::new("z", SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )
                ),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()])
        );
    }

    #[test]
    fn convert_non_variable_applications_wrapped_by_another_applications() {
        assert_eq!(
            desugar_non_variable_applications(&Module::from_definitions(vec![
                ValueDefinition::new(
                    "x",
                    Application::new(
                        Application::new(
                            Let::new(
                                vec![FunctionDefinition::new(
                                    "f",
                                    vec!["y".into()],
                                    Variable::new("y", SourceInformation::dummy()),
                                    types::Function::new(
                                        types::Number::new(SourceInformation::dummy()),
                                        types::Number::new(SourceInformation::dummy()),
                                        SourceInformation::dummy()
                                    ),
                                    SourceInformation::dummy(),
                                )
                                .into()],
                                Variable::new("f", SourceInformation::dummy())
                            ),
                            Variable::new("z", SourceInformation::dummy()),
                            SourceInformation::dummy()
                        ),
                        Variable::new("z", SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()
            ])),
            Module::from_definitions(vec![ValueDefinition::new(
                "x",
                Let::new(
                    vec![FunctionDefinition::new(
                        "f",
                        vec!["y".into()],
                        Variable::new("y", SourceInformation::dummy()),
                        types::Function::new(
                            types::Number::new(SourceInformation::dummy()),
                            types::Number::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        ),
                        SourceInformation::dummy(),
                    )
                    .into()],
                    Application::new(
                        Application::new(
                            Variable::new("f", SourceInformation::dummy()),
                            Variable::new("z", SourceInformation::dummy()),
                            SourceInformation::dummy()
                        ),
                        Variable::new("z", SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )
                ),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()])
        );
    }

    #[test]
    fn do_no_convert_variable_applications() {
        assert_eq!(
            desugar_non_variable_applications(&Module::from_definitions(vec![
                ValueDefinition::new(
                    "x",
                    Application::new(
                        Variable::new("f", SourceInformation::dummy()),
                        Variable::new("y", SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()
            ])),
            Module::from_definitions(vec![ValueDefinition::new(
                "x",
                Application::new(
                    Variable::new("f", SourceInformation::dummy()),
                    Variable::new("y", SourceInformation::dummy()),
                    SourceInformation::dummy()
                ),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()])
        );
    }
}
