#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FilePath {
    components: Vec<String>,
}

impl FilePath {
    pub const fn new(components: Vec<String>) -> Self {
        Self { components }
    }

    pub fn components(&self) -> &[String] {
        &self.components
    }

    pub fn with_extension(&self, extension: &str) -> Self {
        let replacement = if extension == "" {
            "".into()
        } else {
            format!(".{}", extension)
        };

        Self::new(
            self.components()[..(self.components.len() - 1)]
                .iter()
                .chain(&[regex::Regex::new(r"(\..*)?$")
                    .unwrap()
                    .replace(
                        &self.components.iter().last().unwrap(),
                        replacement.as_str(),
                    )
                    .into()])
                .cloned()
                .collect(),
        )
    }
}

impl std::fmt::Display for FilePath {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self.components().join("/"))
    }
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(format!("{}", FilePath::new(vec!["foo"])), "foo");
        assert_eq!(format!("{}", FilePath::new(vec!["foo", "bar"])), "foo/bar");
    }

    #[test]
    fn with_extension() {
        assert_eq!(
            FilePath::new(vec!["foo"]).with_extension("c"),
            FilePath::new(vec!["foo.c"])
        );
        assert_eq!(
            FilePath::new(vec!["foo", "bar"]).with_extension("c"),
            FilePath::new(vec!["foo", "bar.c"])
        );
        assert_eq!(
            FilePath::new(vec!["foo.c"]).with_extension(""),
            FilePath::new(vec!["foo"])
        );
        assert_eq!(
            FilePath::new(vec!["foo.c"]).with_extension("h"),
            FilePath::new(vec!["foo.h"])
        );
    }
}
