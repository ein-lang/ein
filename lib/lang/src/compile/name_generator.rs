use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
pub struct NameGenerator {
    index: AtomicUsize,
    prefix: String,
}

impl NameGenerator {
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            index: AtomicUsize::new(0),
            prefix: prefix.into(),
        }
    }

    pub fn generate(&self) -> String {
        let index = self.index.fetch_add(1, Ordering::SeqCst);
        format!("{}{}", self.prefix, index)
    }
}

#[cfg(test)]
mod tests {
    use super::NameGenerator;

    #[test]
    fn generate_name() {
        let generator = NameGenerator::new("name");

        assert_eq!(generator.generate(), "name0");
        assert_eq!(generator.generate(), "name1");
    }
}
