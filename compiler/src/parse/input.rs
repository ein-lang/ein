use crate::debug::Location;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Input<'a> {
    source: &'a str,
    braces: usize,
    location: Location,
}

impl<'a> Input<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            braces: 0,
            location: Location::default(),
        }
    }

    pub(super) fn with_metadata(source: &'a str, braces: usize, location: Location) -> Self {
        Self {
            source,
            braces,
            location,
        }
    }

    pub fn source(&self) -> &'a str {
        self.source
    }

    pub fn braces(&self) -> usize {
        self.braces
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn set_braces(&self, braces: usize) -> Self {
        Self::with_metadata(self.source, braces, self.location)
    }
}
