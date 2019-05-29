use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct CompileError {
    message: String,
}

impl CompileError {
    pub fn new(message: String) -> Self {
        CompileError { message }
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
