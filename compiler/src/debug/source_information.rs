use super::location::Location;

#[derive(Clone, Debug, PartialEq)]
struct SourceInformation {
    line: String,
    location: Location,
}

impl SourceInformation {
    pub fn new(line: String, location: Location) -> Self {
        Self { line, location }
    }

    pub fn line(&self) -> &str {
        &self.line
    }

    pub fn location(&self) -> &Location {
        &self.location
    }
}
