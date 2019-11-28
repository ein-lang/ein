#[derive(Clone, Copy, Debug)]
pub enum PackageConfigurationError {
    CommandNameForLibrary,
    MissingCommandName,
}

impl std::error::Error for PackageConfigurationError {}

impl std::fmt::Display for PackageConfigurationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::CommandNameForLibrary => {
                write!(formatter, "command name not allowed for library target")
            }
            Self::MissingCommandName => {
                write!(formatter, "command name missing in package configuration")
            }
        }
    }
}
