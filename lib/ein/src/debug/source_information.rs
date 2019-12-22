use super::location::Location;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceInformation {
    source_name: String,
    location: Location,
    line: String,
}

impl SourceInformation {
    pub fn new(
        source_name: impl Into<String>,
        location: Location,
        line: impl Into<String>,
    ) -> Self {
        Self {
            source_name: source_name.into(),
            location,
            line: line.into(),
        }
    }

    #[cfg(test)]
    pub fn dummy() -> Self {
        Self::new("", Location::new(0, 0), "")
    }
}

impl Display for SourceInformation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let line_information = format!(
            "{}:{}:",
            self.location.line_number(),
            self.location.column_number()
        );

        write!(
            formatter,
            "{}\n{}\t{}\n{}\t{}^",
            self.source_name,
            &line_information,
            self.line,
            str::repeat(" ", line_information.len()),
            str::repeat(" ", self.location.column_number() - 1),
        )
    }
}

impl PartialEq for SourceInformation {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Hash for SourceInformation {
    fn hash<H: Hasher>(&self, _: &mut H) {}
}

#[cfg(test)]
mod tests {
    use super::{Location, SourceInformation};

    #[test]
    fn display() {
        assert_eq!(
            format!(
                "{}",
                SourceInformation::new("file", Location::new(1, 1), "x")
            ),
            "file\n1:1:\tx\n    \t^"
        );

        assert_eq!(
            format!(
                "{}",
                SourceInformation::new("file", Location::new(1, 2), " x")
            ),
            "file\n1:2:\t x\n    \t ^"
        );
    }
}
