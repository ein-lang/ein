mod combinators;
mod error;
mod input;
mod utilities;

use error::ParseError;
use input::Input;
use nom::Err;

pub fn parse(source: &str, filename: &str) -> Result<crate::ast::Module, error::ParseError> {
    combinators::module(Input::new(source, filename))
        .map(|(_, module)| module)
        .map_err(|error| match error {
            Err::Error((input, _)) => ParseError::new(&input),
            Err::Failure((input, _)) => ParseError::new(&input),
            Err::Incomplete(_) => ParseError::new(&Input::new(source, filename)),
        })
}

#[cfg(test)]
mod test {
    use super::parse;
    use crate::ast::*;
    use crate::debug::SourceInformation;
    use crate::types;
    use indoc::indoc;

    #[test]
    fn parse_module() {
        assert_eq!(
            parse("foo : Number -> Number -> Number\nfoo x y = 42", ""),
            Ok(Module::without_exported_names(vec![
                FunctionDefinition::new(
                    "foo",
                    vec!["x".into(), "y".into()],
                    Number::new(42.0, SourceInformation::dummy()),
                    types::Function::new(
                        types::Number::new(SourceInformation::dummy()),
                        types::Function::new(
                            types::Number::new(SourceInformation::dummy()),
                            types::Number::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        ),
                        SourceInformation::dummy()
                    ),
                    SourceInformation::dummy()
                )
                .into()
            ]))
        );

        assert_eq!(
            parse("x : Number\nx = (let x = 42\nin x)", ""),
            Ok(Module::without_exported_names(vec![ValueDefinition::new(
                "x",
                Let::new(
                    vec![ValueDefinition::new(
                        "x",
                        Number::new(42.0, SourceInformation::dummy()),
                        types::Variable::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )
                    .into()],
                    Variable::new("x", SourceInformation::dummy())
                ),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy()
            )
            .into()]))
        );

        assert_eq!(
            parse(
                indoc!(
                    "
                    main : Number -> Number
                    main x = (
                        let
                            f x = x
                            y = (
                                f x
                            )
                        in
                            y
                    )
                "
                ),
                ""
            ),
            Ok(Module::without_exported_names(vec![
                FunctionDefinition::new(
                    "main",
                    vec!["x".into(),],
                    Let::new(
                        vec![
                            FunctionDefinition::new(
                                "f",
                                vec!["x".into(),],
                                Variable::new("x", SourceInformation::dummy()),
                                types::Function::new(
                                    types::Variable::new(SourceInformation::dummy()),
                                    types::Variable::new(SourceInformation::dummy()),
                                    SourceInformation::dummy()
                                ),
                                SourceInformation::dummy()
                            )
                            .into(),
                            ValueDefinition::new(
                                "y",
                                Application::new(
                                    Variable::new("f", SourceInformation::dummy()),
                                    Variable::new("x", SourceInformation::dummy()),
                                    SourceInformation::dummy()
                                ),
                                types::Variable::new(SourceInformation::dummy()),
                                SourceInformation::dummy()
                            )
                            .into()
                        ],
                        Variable::new("y", SourceInformation::dummy())
                    ),
                    types::Function::new(
                        types::Number::new(SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    SourceInformation::dummy()
                )
                .into()
            ]))
        );
    }
}
