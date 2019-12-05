use std::error::Error;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum InfrastructureError {
    HostNotFound,
    LlvmLinkNotFound,
    OriginUrlNotFound,
    CommandExit { status_code: Option<i32> },
}

impl Error for InfrastructureError {}

impl Display for InfrastructureError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::HostNotFound => write!(formatter, "host name for package name not defined"),
            Self::LlvmLinkNotFound => write!(formatter, "llvm-link not found"),
            Self::OriginUrlNotFound => write!(formatter, "repository origin URL not defined"),
            Self::CommandExit { status_code } => match status_code {
                Some(status_code) => {
                    write!(formatter, "command exited with status code {}", status_code)
                }
                None => write!(formatter, "command exited without status code"),
            },
        }
    }
}
