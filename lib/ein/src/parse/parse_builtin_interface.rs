use super::error::ParseError;
use super::parsers::{builtin_interface, stream};
use crate::ast::*;
use combine::Parser;

pub fn parse_builtin_interface(
    source_content: &str,
    source_name: &str,
) -> Result<BuiltinInterface, ParseError> {
    Ok(builtin_interface()
        .parse(stream(source_content, source_name))
        .map(|(module, _)| module)
        .map_err(|error| ParseError::new(source_name, &error))?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug::SourceInformation;
    use crate::types;

    #[test]
    fn parse_empty_interface() {
        assert_eq!(
            parse_builtin_interface("", ""),
            Ok(BuiltinInterface::new(
                Default::default(),
                Default::default()
            ))
        );
    }

    #[test]
    fn parse_function_declaration() {
        assert_eq!(
            parse_builtin_interface("foo : Number -> Number", ""),
            Ok(BuiltinInterface::new(
                Default::default(),
                vec![(
                    "foo".into(),
                    types::Function::new(
                        types::Number::new(SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )
                )]
                .into_iter()
                .collect()
            ))
        );
    }

    #[test]
    fn parse_function_declarations() {
        assert_eq!(
            parse_builtin_interface("foo : Number -> Number\nbar : Number -> Number", ""),
            Ok(BuiltinInterface::new(
                Default::default(),
                vec![
                    (
                        "foo".into(),
                        types::Function::new(
                            types::Number::new(SourceInformation::dummy()),
                            types::Number::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        )
                    ),
                    (
                        "bar".into(),
                        types::Function::new(
                            types::Number::new(SourceInformation::dummy()),
                            types::Number::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        )
                    )
                ]
                .into_iter()
                .collect()
            ))
        );
    }

    #[test]
    fn parse_type_definition() {
        assert_eq!(
            parse_builtin_interface("type foo = Number", ""),
            Ok(BuiltinInterface::new(
                vec![(
                    "foo".into(),
                    types::Number::new(SourceInformation::dummy()).into()
                )]
                .into_iter()
                .collect(),
                Default::default(),
            ))
        );
    }

    #[test]
    fn parse_type_definitions() {
        assert_eq!(
            parse_builtin_interface("type foo = Number\ntype bar = Number", ""),
            Ok(BuiltinInterface::new(
                vec![
                    (
                        "foo".into(),
                        types::Number::new(SourceInformation::dummy()).into()
                    ),
                    (
                        "bar".into(),
                        types::Number::new(SourceInformation::dummy()).into()
                    )
                ]
                .into_iter()
                .collect(),
                Default::default(),
            ))
        );
    }

    #[test]
    fn parse_type_definition_and_function_declarations() {
        assert_eq!(
            parse_builtin_interface("type foo = Number\nbar : Number -> Number", ""),
            Ok(BuiltinInterface::new(
                vec![(
                    "foo".into(),
                    types::Number::new(SourceInformation::dummy()).into()
                )]
                .into_iter()
                .collect(),
                vec![(
                    "bar".into(),
                    types::Function::new(
                        types::Number::new(SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )
                )]
                .into_iter()
                .collect()
            ))
        );
    }

    #[test]
    fn fail_to_parse_interface_with_variable() {
        assert!(matches!(
            parse_builtin_interface("foo : Number", ""),
            Err(_)
        ));
    }
}
