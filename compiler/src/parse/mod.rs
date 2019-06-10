mod combinators;
mod error;
mod input;
mod utilities;

use error::ParseError;
use input::Input;

pub fn parse(source: &str) -> Result<crate::ast::Module, error::ParseError> {
    combinators::module(Input::new(source, 0))
        .map(|(_, module)| module)
        .map_err(|_| ParseError::new("syntax error".into()))
}

#[cfg(test)]
mod test {
    use super::parse;
    use crate::ast::*;
    use crate::types::{self, Type};

    #[test]
    fn parse_module() {
        assert_eq!(
            parse("foo : Number -> Number -> Number\nfoo x y = 42").unwrap(),
            Module::new(vec![FunctionDefinition::new(
                "foo".into(),
                vec!["x".into(), "y".into()],
                42.0.into(),
                types::Function::new(
                    Type::Number,
                    types::Function::new(Type::Number, Type::Number).into()
                )
            )
            .into()])
        );
    }
}
