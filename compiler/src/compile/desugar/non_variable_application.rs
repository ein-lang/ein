use super::super::name_generator::NameGenerator;
use crate::ast::*;
use crate::types;

pub fn desugar_non_variable_applications(module: &Module) -> Module {
    let mut name_generator = NameGenerator::new("generated_function_");

    module.convert_expressions(&mut |expression| match expression {
        Expression::Application(application) => match application.function() {
            Expression::Let(let_) => {
                let function_name = name_generator.generate();
                let source_information = application.source_information();

                Let::new(
                    vec![ValueDefinition::new(
                        function_name.clone(),
                        let_.clone(),
                        types::Variable::new(source_information.clone()),
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
            _ => application.clone().into(),
        },
        _ => expression.clone(),
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::debug::*;
    use crate::types;

    #[test]
    fn convert_non_variable_applications() {
        assert_eq!(
            desugar_non_variable_applications(&Module::without_exported_names(vec![
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
            Module::without_exported_names(vec![ValueDefinition::new(
                "x",
                Let::new(
                    vec![ValueDefinition::new(
                        "generated_function_0",
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
                        types::Variable::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    )
                    .into()],
                    Application::new(
                        Variable::new("generated_function_0", SourceInformation::dummy()),
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
            desugar_non_variable_applications(&Module::without_exported_names(vec![
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
            Module::without_exported_names(vec![ValueDefinition::new(
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
