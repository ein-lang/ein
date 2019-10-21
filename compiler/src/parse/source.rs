#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Source<'a> {
    name: &'a str,
    content: &'a str,
}

impl<'a> Source<'a> {
    pub fn new(name: &'a str, content: &'a str) -> Self {
        Self { name, content }
    }

    pub fn name(&self) -> &'a str {
        &self.name
    }

    pub fn content(&self) -> &'a str {
        &self.content
    }
}
