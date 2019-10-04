use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct BuildError {
    error: std::io::Error,
}

impl Display for BuildError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            formatter,
            "IOError: {}\n\nBuildError: Failed to build due to the reason above",
            self.error
        )
    }
}

impl Error for BuildError {}

impl From<std::io::Error> for BuildError {
    fn from(error: std::io::Error) -> Self {
        Self { error }
    }
}

#[cfg(test)]
mod test {
    use super::BuildError;

    #[test]
    fn display_message() {
        assert_eq!(
            format!(
                "{}",
                BuildError::from(std::io::Error::new(std::io::ErrorKind::Other, "foo"))
            ),
            "IOError: foo\n\nBuildError: Failed to build due to the reason above"
        );
    }
}
