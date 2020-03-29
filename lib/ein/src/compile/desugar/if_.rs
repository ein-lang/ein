use crate::ast::*;

pub fn desugar_if(module: &Module) -> Module {
    module.convert_expressions(&mut |expression| match expression {
        Expression::If(if_) => Case::new(
            if_.condition().clone(),
            vec![
                Alternative::new(
                    Boolean::new(true, if_.source_information().clone()),
                    if_.then().clone(),
                ),
                Alternative::new(
                    Boolean::new(false, if_.source_information().clone()),
                    if_.else_().clone(),
                ),
            ],
            if_.source_information().clone(),
        )
        .into(),
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
    fn desugar_if_expressions() {
        assert_eq!(
            desugar_if(&Module::from_definitions(vec![ValueDefinition::new(
                "x",
                If::new(
                    Boolean::new(true, SourceInformation::dummy()),
                    Number::new(1.0, SourceInformation::dummy()),
                    Number::new(2.0, SourceInformation::dummy()),
                    SourceInformation::dummy()
                ),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy()
            )
            .into()])),
            Module::from_definitions(vec![ValueDefinition::new(
                "x",
                Case::new(
                    Boolean::new(true, SourceInformation::dummy()),
                    vec![
                        Alternative::new(
                            Boolean::new(true, SourceInformation::dummy()),
                            Number::new(1.0, SourceInformation::dummy()),
                        ),
                        Alternative::new(
                            Boolean::new(false, SourceInformation::dummy()),
                            Number::new(2.0, SourceInformation::dummy()),
                        ),
                    ],
                    SourceInformation::dummy(),
                ),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy()
            )
            .into()])
        );
    }
}
