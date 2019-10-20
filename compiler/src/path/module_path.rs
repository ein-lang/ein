#[derive(Clone, Debug, PartialEq)]
pub enum ModulePath {
    External(Vec<String>),
    Internal(Vec<String>),
}

impl ModulePath {
    pub fn qualify_name(&self, name: &str) -> String {
        let path = match self {
            Self::External(components) => components.join("."),
            Self::Internal(components) => format!(".{}", components.join(".")),
        };

        [&path, name].join(".")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn qualify_name_with_external_path() {
        assert_eq!(
            ModulePath::External(vec!["foo".into()]).qualify_name("bar"),
            "foo.bar"
        );
    }

    #[test]
    fn qualify_name_with_internal_path() {
        assert_eq!(
            ModulePath::Internal(vec!["foo".into()]).qualify_name("bar"),
            ".foo.bar"
        );
    }
}
