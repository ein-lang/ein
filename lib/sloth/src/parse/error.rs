use super::input::Input;
use crate::debug::*;
use std::error::Error;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub struct ParseError {
    source_information: Box<SourceInformation>,
}

impl ParseError {
    pub fn new(input: &Input) -> Self {
        ParseError {
            source_information: SourceInformation::new(
                input.source().name(),
                input.location(),
                input.line(),
            )
            .into(),
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            formatter,
            "ParseError: Failed to parse\n{}",
            self.source_information,
        )
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        "ParseError"
    }
}
