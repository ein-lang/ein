use std::error::Error;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct CompileError {
    message: String,
}

impl CompileError {
    pub fn new(message: &str) -> Self {
        CompileError {
            message: message.into(),
        }
    }
}

impl Display for CompileError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}", self.message)
    }
}

impl Error for CompileError {
    fn description(&self) -> &str {
        &self.message
    }
}
