#[derive(Clone, Debug, PartialEq)]
pub struct ModulePath {
    components: Vec<String>,
}

impl ModulePath {
    pub fn new(components: Vec<String>) -> Self {
        Self { components }
    }

    pub fn components(&self) -> &[String] {
        &self.components
    }

    pub fn qualify_name(&self, name: &str) -> String {
        [&self.components.join("."), name].join(".")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn qualify_name() {
        assert_eq!(
            ModulePath::new(vec!["foo".into()]).qualify_name("bar"),
            "foo.bar"
        );
    }
}
