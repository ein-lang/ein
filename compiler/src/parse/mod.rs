mod combinators;
mod error;
mod input;
mod utilities;

use crate::debug::Location;
use error::ParseError;
use input::Input;
use nom::Err;

pub fn parse(source: &str) -> Result<crate::ast::Module, error::ParseError> {
    combinators::module(Input::new(source))
        .map(|(_, module)| module)
        .map_err(|error| match error {
            Err::Error((input, _)) => ParseError::new(input.location()),
            Err::Failure((input, _)) => ParseError::new(input.location()),
            Err::Incomplete(_) => ParseError::new(Location::default()),
        })
}

#[cfg(test)]
mod test {
    use super::parse;
    use crate::ast::*;
    use crate::types::{self, Type};
    use indoc::indoc;

    #[test]
    fn parse_module() {
        assert_eq!(
            parse("foo : Number -> Number -> Number\nfoo x y = 42"),
            Ok(Module::new(vec![FunctionDefinition::new(
                "foo".into(),
                vec!["x".into(), "y".into()],
                42.0.into(),
                types::Function::new(
                    Type::Number,
                    types::Function::new(Type::Number, Type::Number).into()
                )
            )
            .into()]))
        );
        assert_eq!(
            parse("x : Number\nx = (let x = 42\nin x)"),
            Ok(Module::new(vec![ValueDefinition::new(
                "x".into(),
                Let::new(
                    vec![ValueDefinition::new(
                        "x".into(),
                        Expression::Number(42.0),
                        types::Variable::new().into()
                    )
                    .into()],
                    Expression::Variable("x".into())
                )
                .into(),
                Type::Number
            )
            .into()]))
        );
        assert_eq!(
            parse(indoc!(
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
            )),
            Ok(Module::new(vec![FunctionDefinition::new(
                "main".into(),
                vec!["x".into(),],
                Let::new(
                    vec![
                        FunctionDefinition::new(
                            "f".into(),
                            vec!["x".into(),],
                            Expression::Variable("x".into()),
                            types::Function::new(
                                types::Variable::new().into(),
                                types::Variable::new().into()
                            )
                        )
                        .into(),
                        ValueDefinition::new(
                            "y".into(),
                            Application::new(
                                Expression::Variable("f".into()),
                                Expression::Variable("x".into())
                            )
                            .into(),
                            types::Variable::new().into()
                        )
                        .into()
                    ],
                    Expression::Variable("y".into())
                )
                .into(),
                types::Function::new(Type::Number, Type::Number)
            )
            .into()]))
        );
    }
}
