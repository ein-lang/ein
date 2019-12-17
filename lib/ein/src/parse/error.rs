use combine::easy::Errors;
use std::error::Error;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub struct ParseError(String);

impl Display for ParseError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}", self.0)
    }
}

impl Error for ParseError {}

impl<I: Display, R: Display, P: Display> From<Errors<I, R, P>> for ParseError {
    fn from(errors: Errors<I, R, P>) -> Self {
        Self(format!("{}", errors))
    }
}
