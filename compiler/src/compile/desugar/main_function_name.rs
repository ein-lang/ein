use crate::ast::*;

const MAIN_FUNCTION_NAME: &str = "sloth_main";

pub fn desugar_main_function_name(module: &Module) -> Module {
    Module::new(
        module
            .definitions()
            .iter()
            .map(|definition| match definition {
                Definition::FunctionDefinition(function_definition) => FunctionDefinition::new(
                    convert_function_name(function_definition.name()),
                    function_definition.arguments().to_vec(),
                    function_definition.body().clone(),
                    function_definition.type_().clone(),
                    function_definition.source_information().clone(),
                )
                .into(),
                Definition::ValueDefinition(value_definition) => ValueDefinition::new(
                    convert_function_name(value_definition.name()),
                    value_definition.body().clone(),
                    value_definition.type_().clone(),
                    value_definition.source_information().clone(),
                )
                .into(),
            })
            .collect(),
    )
}

fn convert_function_name(name: &str) -> &str {
    if name == "main" {
        MAIN_FUNCTION_NAME
    } else {
        name
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::debug::*;
    use crate::types;

    #[test]
    fn convert_name_of_function_definition() {
        assert_eq!(
            desugar_main_function_name(&Module::new(vec![FunctionDefinition::new(
                "main",
                vec!["x".into()],
                Number::new(42.0, SourceInformation::dummy()),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()])),
            Module::new(vec![FunctionDefinition::new(
                "sloth_main",
                vec!["x".into()],
                Number::new(42.0, SourceInformation::dummy()),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()])
        );
    }

    #[test]
    fn convert_name_of_value_definition() {
        assert_eq!(
            desugar_main_function_name(&Module::new(vec![ValueDefinition::new(
                "main",
                Number::new(42.0, SourceInformation::dummy()),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()])),
            Module::new(vec![ValueDefinition::new(
                "sloth_main",
                Number::new(42.0, SourceInformation::dummy()),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()])
        );
    }

    #[test]
    fn do_not_convert_non_main_name() {
        assert_eq!(
            desugar_main_function_name(&Module::new(vec![FunctionDefinition::new(
                "mainish",
                vec!["x".into()],
                Number::new(42.0, SourceInformation::dummy()),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()])),
            Module::new(vec![FunctionDefinition::new(
                "mainish",
                vec!["x".into()],
                Number::new(42.0, SourceInformation::dummy()),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()])
        );
    }
}
