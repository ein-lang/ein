#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Input<'a> {
    source: &'a str,
    braces: u64,
}

impl<'a> Input<'a> {
    pub fn new(source: &'a str, braces: u64) -> Self {
        Self { source, braces }
    }

    pub fn source(&self) -> &'a str {
        self.source
    }

    pub fn braces(&self) -> u64 {
        self.braces
    }
}
