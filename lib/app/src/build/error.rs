#[derive(Debug)]
pub enum BuildError {
    CircularDepdendency,
}

impl std::error::Error for BuildError {}

impl std::fmt::Display for BuildError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "circular module dependency detected")
    }
}
