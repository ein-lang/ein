mod error;

use std::error::Error;

lalrpop_mod!(syntax, "/parse/syntax.rs");

pub fn parse(source: &str) -> Result<crate::ast::Expression, error::ParseError> {
    syntax::ExpressionParser::new()
        .parse(source)
        .map_err(|err| error::ParseError::new(err.description().into()))
}

#[cfg(test)]
mod test {
    use super::parse;
    use crate::ast::{Application, Expression, Operator};

    #[test]
    fn parse_number() {
        assert_eq!(parse("42").unwrap(), Expression::Number(42.0));
        assert_eq!(parse("-42").unwrap(), Expression::Number(-42.0));
        assert_eq!(parse("4.2").unwrap(), Expression::Number(4.2));
    }

    #[test]
    fn parse_application() {
        assert_eq!(
            parse("(+ 1 2)").unwrap(),
            Application::new(Operator::Add, 1.0.into(), 2.0.into()).into()
        );
        assert_eq!(
            parse("(- (* 1 2) (/ 3 4))").unwrap(),
            Application::new(
                Operator::Subtract,
                Application::new(Operator::Multiply, 1.0.into(), 2.0.into()).into(),
                Application::new(Operator::Divide, 3.0.into(), 4.0.into()).into()
            )
            .into()
        );
    }
}
