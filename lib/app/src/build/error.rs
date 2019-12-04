#[derive(Debug, PartialEq)]
pub enum BuildError {
    CircularDependency,
    ExternalPackageConfigurationFileNotFound { package_name: String },
}

impl std::error::Error for BuildError {}

impl std::fmt::Display for BuildError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::CircularDependency => write!(formatter, "circular module dependency detected"),
            Self::ExternalPackageConfigurationFileNotFound { package_name } => write!(
                formatter,
                "package configuration file not found in external package \"{}\"",
                package_name
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", BuildError::CircularDependency),
            "circular module dependency detected"
        );
        assert_eq!(
            format!(
                "{}",
                BuildError::ExternalPackageConfigurationFileNotFound {
                    package_name: "foo".into()
                }
            ),
            "package configuration file not found in external package \"foo\""
        );
    }
}
