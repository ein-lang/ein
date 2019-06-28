use crate::debug::Location;
use std::error::Error;
use std::fmt::Display;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParseError {
    location: Location,
}

impl ParseError {
    pub fn new(location: Location) -> Self {
        ParseError { location }
    }
}

impl Display for ParseError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            formatter,
            "ParseError:{}:{}",
            self.location.line_number(),
            self.location.column_number()
        )
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        "parse error"
    }
}
