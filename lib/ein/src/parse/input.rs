use super::source::Source;
use crate::debug::Location;
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Input<'a> {
    source: Source<'a>,
    braces: usize,
    location: Location,
    lines: Rc<[&'a str]>,
}

impl<'a> Input<'a> {
    pub fn new(source: Source<'a>) -> Self {
        Self {
            source,
            braces: 0,
            location: Location::default(),
            lines: source.content().split('\n').collect::<Vec<_>>().into(),
        }
    }

    pub fn set(&self, source_content: &'a str, braces: usize, location: Location) -> Self {
        Self {
            source: Source::new(self.source.name(), source_content),
            braces,
            location,
            lines: self.lines.clone(),
        }
    }

    pub fn source(&self) -> Source<'a> {
        self.source
    }

    pub fn braces(&self) -> usize {
        self.braces
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn line(&self) -> &str {
        &self.lines[self.location.line_number() - 1]
    }

    pub fn set_braces(&self, braces: usize) -> Self {
        self.set(self.source.content(), braces, self.location)
    }
}

impl<'a> nom::InputLength for Input<'a> {
    fn input_len(&self) -> usize {
        self.source.content().input_len()
    }
}
