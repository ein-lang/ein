use crate::debug::Location;
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Input<'a> {
    source: &'a str,
    braces: usize,
    location: Location,
    filename: &'a str,
    lines: Rc<[&'a str]>,
}

impl<'a> Input<'a> {
    pub fn new(source: &'a str, filename: &'a str) -> Self {
        Self {
            source,
            braces: 0,
            location: Location::default(),
            filename,
            lines: source.split("\n").collect::<Vec<_>>().into(),
        }
    }

    pub fn from_str(&self, source: &'a str, braces: usize, location: Location) -> Self {
        Self {
            source,
            braces,
            location,
            filename: self.filename,
            lines: self.lines.clone(),
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

    pub fn filename(&self) -> &str {
        self.filename
    }

    pub fn line(&self) -> &str {
        &self.lines[self.location.line_number() - 1]
    }

    pub fn set_braces(&self, braces: usize) -> Self {
        self.from_str(self.source, braces, self.location)
    }
}
