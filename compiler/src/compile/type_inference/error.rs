use std::error::Error;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub struct TypeInferenceError {
    message: String,
}

impl TypeInferenceError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for TypeInferenceError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}", self.message)
    }
}

impl Error for TypeInferenceError {
    fn description(&self) -> &str {
        &self.message
    }
}
