use super::input::Input;
use crate::debug::Location;
use std::error::Error;
use std::fmt::Display;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseError {
    filename: String,
    location: Location,
    line: String,
}

impl ParseError {
    pub fn new(input: &Input) -> Self {
        ParseError {
            filename: input.filename().into(),
            location: input.location(),
            line: input.line().into(),
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            formatter,
            "ParseError:{}:{}:{}:\t{}",
            self.filename,
            self.location.line_number(),
            self.location.column_number(),
            self.line,
        )
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        "ParseError"
    }
}
