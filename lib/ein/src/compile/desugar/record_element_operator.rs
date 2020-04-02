use super::super::name_generator::NameGenerator;
use crate::ast::*;
use crate::types;

pub fn desugar_record_element_operators(module: &Module) -> Module {
    let mut function_name_generator = NameGenerator::new("record_element_operator_function_");
    let mut argument_name_generator = NameGenerator::new("record_element_operator_argument_");

    module.convert_expressions(&mut |expression| match expression {
        Expression::RecordElementOperator(operator) => {
            let function_name = function_name_generator.generate();
            let argument_name = argument_name_generator.generate();
            let source_information = operator.source_information();

            Let::new(
                vec![FunctionDefinition::new(
                    function_name.clone(),
                    vec![argument_name.clone()],
                    RecordElementOperation::new(
                        operator.key(),
                        Variable::new(argument_name, source_information.clone()),
                        types::Unknown::new(source_information.clone()),
                        source_information.clone(),
                    ),
                    types::Unknown::new(source_information.clone()),
                    source_information.clone(),
                )
                .into()],
                Variable::new(function_name, source_information.clone()),
            )
            .into()
        }
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
    fn desugar_record_element_operator() {
        assert_eq!(
            desugar_record_element_operators(&Module::from_definitions(vec![
                ValueDefinition::new(
                    "x",
                    RecordElementOperator::new("foo", SourceInformation::dummy()),
                    types::Unknown::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()
            ])),
            Module::from_definitions(vec![ValueDefinition::new(
                "x",
                Let::new(
                    vec![FunctionDefinition::new(
                        "record_element_operator_function_0",
                        vec!["record_element_operator_argument_0".into()],
                        RecordElementOperation::new(
                            "foo",
                            Variable::new(
                                "record_element_operator_argument_0",
                                SourceInformation::dummy()
                            ),
                            types::Unknown::new(SourceInformation::dummy()),
                            SourceInformation::dummy(),
                        ),
                        types::Unknown::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    )
                    .into()],
                    Variable::new(
                        "record_element_operator_function_0",
                        SourceInformation::dummy()
                    ),
                ),
                types::Unknown::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()])
        );
    }
}
