use super::super::name_generator::NameGenerator;
use crate::ast::*;
use crate::debug::*;
use crate::types::Type;

pub fn desugar_argument_omission(module: &Module) -> Module {
    let mut name_generator = NameGenerator::new("omitted_argument_");

    module.convert_definitions(&mut |definition| match definition {
        Definition::FunctionDefinition(function_definition) => function_definition.clone().into(),
        Definition::ValueDefinition(value_definition) => match value_definition.type_() {
            Type::Function(function_type) => {
                let arguments = function_type
                    .arguments()
                    .iter()
                    .map(|_| name_generator.generate())
                    .collect::<Vec<_>>();

                FunctionDefinition::new(
                    value_definition.name().clone(),
                    arguments.clone(),
                    append_arguments_to_expression(
                        value_definition.body(),
                        &arguments
                            .iter()
                            .map(|argument| {
                                Variable::new(
                                    argument,
                                    value_definition.source_information().clone(),
                                )
                            })
                            .collect::<Vec<Variable>>(),
                    ),
                    value_definition.type_().clone(),
                    value_definition.source_information().clone(),
                )
                .into()
            }
            _ => value_definition.clone().into(),
        },
    })
}

fn append_arguments_to_expression(expression: &Expression, arguments: &[Variable]) -> Expression {
    match expression {
        Expression::Application(application) => {
            append_arguments_to_expression_with_source_information(
                expression,
                arguments,
                application.source_information(),
            )
        }
        Expression::Let(let_) => Let::new(
            let_.definitions().to_vec(),
            append_arguments_to_expression(let_.expression(), arguments),
        )
        .into(),
        Expression::Variable(variable) => append_arguments_to_expression_with_source_information(
            expression,
            arguments,
            variable.source_information(),
        ),
        _ => unreachable!(),
    }
}

fn append_arguments_to_expression_with_source_information(
    expression: &Expression,
    arguments: &[Variable],
    source_information: &SourceInformation,
) -> Expression {
    arguments
        .iter()
        .fold(expression.clone(), |application, argument| {
            Application::new(
                application.clone(),
                argument.clone(),
                source_information.clone(),
            )
            .into()
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types;

    #[test]
    fn complement_an_omitted_argument_of_value_definition() {
        assert_eq!(
            desugar_argument_omission(&Module::new(vec![ValueDefinition::new(
                "f",
                Variable::new("g", SourceInformation::dummy()),
                types::Function::new(
                    types::Number::new(SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy()
                ),
                SourceInformation::dummy(),
            )
            .into()])),
            Module::new(vec![FunctionDefinition::new(
                "f",
                vec!["omitted_argument_0".into()],
                Application::new(
                    Variable::new("g", SourceInformation::dummy()),
                    Variable::new("omitted_argument_0", SourceInformation::dummy()),
                    SourceInformation::dummy()
                ),
                types::Function::new(
                    types::Number::new(SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy()
                ),
                SourceInformation::dummy(),
            )
            .into()])
        );
    }

    #[test]
    fn complement_2_omitted_arguments_of_value_definition() {
        assert_eq!(
            desugar_argument_omission(&Module::new(vec![ValueDefinition::new(
                "f",
                Variable::new("g", SourceInformation::dummy()),
                types::Function::new(
                    types::Number::new(SourceInformation::dummy()),
                    types::Function::new(
                        types::Number::new(SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    SourceInformation::dummy()
                ),
                SourceInformation::dummy(),
            )
            .into()])),
            Module::new(vec![FunctionDefinition::new(
                "f",
                vec!["omitted_argument_0".into(), "omitted_argument_1".into()],
                Application::new(
                    Application::new(
                        Variable::new("g", SourceInformation::dummy()),
                        Variable::new("omitted_argument_0", SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    Variable::new("omitted_argument_1", SourceInformation::dummy()),
                    SourceInformation::dummy()
                ),
                types::Function::new(
                    types::Number::new(SourceInformation::dummy()),
                    types::Function::new(
                        types::Number::new(SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    SourceInformation::dummy()
                ),
                SourceInformation::dummy(),
            )
            .into()])
        );
    }
}
