#[derive(Debug)]
pub struct NameGenerator {
    index: usize,
    prefix: String,
}

impl NameGenerator {
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            index: 0,
            prefix: prefix.into(),
        }
    }

    pub fn generate(&mut self) -> String {
        let index = self.index;
        self.index += 1;
        format!("{}{}", self.prefix, index)
    }
}

#[cfg(test)]
mod tests {
    use super::NameGenerator;

    #[test]
    fn generate_name() {
        let mut generator = NameGenerator::new("name");

        assert_eq!(generator.generate(), "name0");
        assert_eq!(generator.generate(), "name1");
    }
}
