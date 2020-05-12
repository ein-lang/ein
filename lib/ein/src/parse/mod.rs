#[macro_use]
mod attempt;
mod error;
mod parsers;
mod utilities;

use crate::ast;
use combine::Parser;
pub use error::ParseError;
use parsers::{module, stream};

pub fn parse(source_content: &str, source_name: &str) -> Result<ast::UnresolvedModule, ParseError> {
    Ok(module()
        .parse(stream(source_content, source_name))
        .map(|(module, _)| module)?)
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
    fn parse_() {
        for (source, expected_module) in vec![
            (
                "foo : Number -> Number -> Number\nfoo x y = 42",
                UnresolvedModule::from_definitions(vec![FunctionDefinition::new(
                    "foo",
                    vec!["x".into(), "y".into()],
                    Number::new(42.0, SourceInformation::dummy()),
                    types::Function::new(
                        types::Number::new(SourceInformation::dummy()),
                        types::Function::new(
                            types::Number::new(SourceInformation::dummy()),
                            types::Number::new(SourceInformation::dummy()),
                            SourceInformation::dummy(),
                        ),
                        SourceInformation::dummy(),
                    ),
                    SourceInformation::dummy(),
                )
                .into()]),
            ),
            (
                "x : Number\nx = (let x = 42\nin x)",
                UnresolvedModule::from_definitions(vec![ValueDefinition::new(
                    "x",
                    Let::new(
                        vec![ValueDefinition::new(
                            "x",
                            Number::new(42.0, SourceInformation::dummy()),
                            types::Unknown::new(SourceInformation::dummy()),
                            SourceInformation::dummy(),
                        )
                        .into()],
                        Variable::new("x", SourceInformation::dummy()),
                    ),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()]),
            ),
            (
                indoc!(
                    "
                    main : Number -> Number
                    main x = (
                        let
                            f x = x
                            y =
                              f x
                        in
                            y
                    )
                    "
                ),
                UnresolvedModule::from_definitions(vec![FunctionDefinition::new(
                    "main",
                    vec!["x".into()],
                    Let::new(
                        vec![
                            FunctionDefinition::new(
                                "f",
                                vec!["x".into()],
                                Variable::new("x", SourceInformation::dummy()),
                                types::Unknown::new(SourceInformation::dummy()),
                                SourceInformation::dummy(),
                            )
                            .into(),
                            ValueDefinition::new(
                                "y",
                                Application::new(
                                    Variable::new("f", SourceInformation::dummy()),
                                    Variable::new("x", SourceInformation::dummy()),
                                    SourceInformation::dummy(),
                                ),
                                types::Unknown::new(SourceInformation::dummy()),
                                SourceInformation::dummy(),
                            )
                            .into(),
                        ],
                        Variable::new("y", SourceInformation::dummy()),
                    ),
                    types::Function::new(
                        types::Number::new(SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    SourceInformation::dummy(),
                )
                .into()]),
            ),
        ] {
            assert_eq!(parse(source, ""), Ok(expected_module));
        }
    }

    #[test]
    fn parse_with_import_statement() {
        assert_eq!(
            parse(
                indoc!(
                    "
                    import \"package/Module\"

                    main : Number -> Number
                    main x = x
                    "
                ),
                ""
            ),
            Ok(UnresolvedModule::new(
                Export::new(Default::default()),
                vec![Import::new(ExternalUnresolvedModulePath::new(vec![
                    "package".into(),
                    "Module".into()
                ]))],
                vec![],
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
    fn parse_with_comment() {
        assert_eq!(
            parse(
                indoc!(
                    "
                    # foo is good
                    foo : Number -> Number
                    foo x = 42
                    "
                ),
                ""
            ),
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
