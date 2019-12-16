mod combinators;
mod error;
mod input;
mod parsers;
mod source;
mod utilities;

use crate::ast;
use error::ParseError;
use input::Input;
use nom::Err;
pub use source::Source;

pub fn parse_module(source: Source) -> Result<ast::UnresolvedModule, ParseError> {
    let content: String;

    let source = if source.content().ends_with('\n') {
        source
    } else {
        content = [source.content(), "\n"].concat();
        Source::new(source.name(), &content)
    };

    combinators::module(Input::new(source))
        .map(|(_, module)| module)
        .map_err(|error| map_error(error, source))
}

fn map_error<T>(error: nom::Err<(Input, T)>, source: Source) -> ParseError {
    match error {
        Err::Error((input, _)) => ParseError::new(&input),
        Err::Failure((input, _)) => ParseError::new(&input),
        Err::Incomplete(_) => ParseError::new(&Input::new(source)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use crate::debug::SourceInformation;
    use crate::path::*;
    use crate::types;
    use indoc::indoc;

    #[test]
    fn parse_module_() {
        assert_eq!(
            parse_module(Source::new(
                "",
                "foo : Number -> Number -> Number\nfoo x y = 42"
            ),),
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
            parse_module(Source::new("", "x : Number\nx = (let x = 42\nin x)")),
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
            parse_module(Source::new(
                "",
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
                )
            )),
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
    fn parse_module_with_import_statement() {
        assert_eq!(
            parse_module(Source::new(
                "",
                indoc!(
                    "
                        import \"package/Module\"

                        main : Number -> Number
                        main x = x
                    "
                )
            ),),
            Ok(UnresolvedModule::new(
                Export::new(Default::default()),
                vec![Import::new(ExternalUnresolvedModulePath::new(vec![
                    "package".into(),
                    "Module".into()
                ]))],
                vec![FunctionDefinition::new(
                    "main",
                    vec!["x".into()],
                    Variable::new("x", SourceInformation::dummy()),
                    types::Function::new(
                        types::Number::new(SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    SourceInformation::dummy()
                )
                .into()]
            ))
        );
    }

    #[test]
    fn parse_module_with_comment() {
        assert_eq!(
            parse_module(Source::new(
                "",
                indoc!(
                    "
                        # foo is good
                        foo : Number -> Number
                        foo x = 42
                    "
                )
            )),
            Ok(UnresolvedModule::from_definitions(vec![
                FunctionDefinition::new(
                    "foo",
                    vec!["x".into()],
                    Number::new(42.0, SourceInformation::dummy()),
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
