mod error;

use std::error::Error;

lalrpop_mod!(syntax, "/parse/syntax.rs");

pub fn parse(source: &str) -> Result<crate::ast::Module, error::ParseError> {
    syntax::ModuleParser::new()
        .parse(source)
        .map_err(|err| error::ParseError::new(err.description().into()))
}

#[cfg(test)]
fn parse_expression(source: &str) -> Result<crate::ast::Expression, error::ParseError> {
    syntax::ExpressionParser::new()
        .parse(source)
        .map_err(|err| error::ParseError::new(err.description().into()))
}

#[cfg(test)]
mod test {
    use super::{parse, parse_expression};
    use crate::ast::{Expression, FunctionDefinition, Module, Operation, Operator};
    use crate::types::{self, Type};

    #[test]
    fn parse_number() {
        assert_eq!(parse_expression("42").unwrap(), Expression::Number(42.0));
        assert_eq!(parse_expression("-42").unwrap(), Expression::Number(-42.0));
        assert_eq!(parse_expression("4.2").unwrap(), Expression::Number(4.2));
    }

    #[test]
    fn parse_operation() {
        assert_eq!(
            parse_expression("1 + 2").unwrap(),
            Operation::new(Operator::Add, 1.0.into(), 2.0.into()).into()
        );
        assert_eq!(
            parse_expression("1 * 2").unwrap(),
            Operation::new(Operator::Multiply, 1.0.into(), 2.0.into()).into()
        );
        assert_eq!(
            parse_expression("1 * 2 - 3").unwrap(),
            Operation::new(
                Operator::Subtract,
                Operation::new(Operator::Multiply, 1.0.into(), 2.0.into()).into(),
                3.0.into()
            )
            .into()
        );
        assert_eq!(
            parse_expression("1 + 2 * 3").unwrap(),
            Operation::new(
                Operator::Add,
                1.0.into(),
                Operation::new(Operator::Multiply, 2.0.into(), 3.0.into()).into(),
            )
            .into()
        );
        assert_eq!(
            parse_expression("1 * 2 - 3 / 4").unwrap(),
            Operation::new(
                Operator::Subtract,
                Operation::new(Operator::Multiply, 1.0.into(), 2.0.into()).into(),
                Operation::new(Operator::Divide, 3.0.into(), 4.0.into()).into()
            )
            .into()
        );
    }

    #[test]
    fn parse_module() {
        assert_eq!(
            parse("foo : Number -> Number -> Number; foo x y = 42;").unwrap(),
            Module::new(vec![FunctionDefinition::new(
                "foo".into(),
                vec!["x".into(), "y".into()],
                42.0.into(),
                types::Function::new(
                    Type::Number,
                    types::Function::new(Type::Number, Type::Number).into()
                )
            )])
        );
    }
}
