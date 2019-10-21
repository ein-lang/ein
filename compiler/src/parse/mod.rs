mod combinators;
mod error;
mod input;
mod utilities;

use crate::ast;
use crate::path::AbsoluteModulePath;
use error::ParseError;
use input::Input;
use nom::Err;

pub fn parse_module(
    source: &str,
    filename: &str,
) -> Result<ast::UnresolvedModule, error::ParseError> {
    combinators::module(Input::new(source, filename))
        .map(|(_, module)| module)
        .map_err(|error| match error {
            Err::Error((input, _)) => ParseError::new(&input),
            Err::Failure((input, _)) => ParseError::new(&input),
            Err::Incomplete(_) => ParseError::new(&Input::new(source, filename)),
        })
}

pub fn parse_absolute_module_path(
    path: &str,
    source_name: &str,
) -> Result<AbsoluteModulePath, error::ParseError> {
    combinators::absolute_module_path(Input::new(path, source_name))
        .map(|(_, module_path)| module_path)
        .map_err(|error| match error {
            Err::Error((input, _)) => ParseError::new(&input),
            Err::Failure((input, _)) => ParseError::new(&input),
            Err::Incomplete(_) => ParseError::new(&Input::new(path, source_name)),
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast::*;
    use crate::debug::SourceInformation;
    use crate::types;
    use indoc::indoc;

    #[test]
    fn parse_module_() {
        assert_eq!(
            parse_module("foo : Number -> Number -> Number\nfoo x y = 42", ""),
            Ok(UnresolvedModule::from_definitions(vec![
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
            parse_module("x : Number\nx = (let x = 42\nin x)", ""),
            Ok(UnresolvedModule::from_definitions(vec![
                ValueDefinition::new(
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
                .into()
            ]))
        );

        assert_eq!(
            parse_module(
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
            Ok(UnresolvedModule::from_definitions(vec![
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

    #[test]
    fn parse_absolute_module_path_() {
        assert_eq!(
            parse_absolute_module_path("foo", ""),
            Ok(AbsoluteModulePath::new(vec!["foo".into()]))
        );
    }

    #[test]
    fn parse_absolute_module_path_with_subpath() {
        assert_eq!(
            parse_absolute_module_path("foo.bar", ""),
            Ok(AbsoluteModulePath::new(vec!["foo".into(), "bar".into()]))
        );
    }
}
