#[derive(Clone, Debug, PartialEq)]
pub enum ModulePath {
    Absolute(Vec<String>),
    Relative(Vec<String>),
}

impl ModulePath {
    pub fn qualify_name(&self, name: &str) -> String {
        let path = match self {
            Self::Absolute(components) => components.join("."),
            Self::Relative(components) => format!(".{}", components.join(".")),
        };

        [&path, name].join(".")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn qualify_name_with_absolute_path() {
        assert_eq!(
            ModulePath::Absolute(vec!["foo".into()]).qualify_name("bar"),
            "foo.bar"
        );
    }

    #[test]
    fn qualify_name_with_relative_path() {
        assert_eq!(
            ModulePath::Relative(vec!["foo".into()]).qualify_name("bar"),
            ".foo.bar"
        );
    }
}
