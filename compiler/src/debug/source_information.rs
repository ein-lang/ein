use super::location::Location;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct SourceInformation {
    filename: String,
    location: Location,
    line: String,
}

impl SourceInformation {
    pub fn new(filename: impl Into<String>, location: Location, line: impl Into<String>) -> Self {
        Self {
            filename: filename.into(),
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
        write!(
            formatter,
            "{}:{}:{}:\t{}\n{}\t{}{}",
            self.filename,
            self.location.line_number(),
            self.location.column_number(),
            self.line,
            str::repeat(
                " ",
                format!(
                    "{}{}{}",
                    self.filename,
                    self.location.line_number(),
                    self.location.column_number()
                )
                .len()
                    + 3
            ),
            str::repeat(" ", self.location.column_number() - 1),
            "^"
        )
    }
}

impl PartialEq for SourceInformation {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use super::{Location, SourceInformation};

    #[test]
    fn display() {
        assert_eq!(
            format!(
                "{}",
                SourceInformation::new("file", Location::new(1, 1), "x")
            ),
            "file:1:1:\tx\n         \t^"
        );

        assert_eq!(
            format!(
                "{}",
                SourceInformation::new("file", Location::new(1, 2), " x")
            ),
            "file:1:2:\t x\n         \t ^"
        );
    }
}
