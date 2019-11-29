use std::ops::Deref;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FilePath {
    components: Vec<String>,
}

impl FilePath {
    pub fn new<I: IntoIterator<Item = impl AsRef<str>>>(components: I) -> Self {
        Self {
            components: components
                .into_iter()
                .map(|string| string.as_ref().into())
                .collect(),
        }
    }

    pub fn components(&self) -> impl Iterator<Item = &str> {
        self.components.iter().map(Deref::deref)
    }

    pub fn with_extension(&self, extension: &str) -> Self {
        let replacement = if extension == "" {
            "".into()
        } else {
            format!(".{}", extension)
        };

        Self::new(
            self.components().take(self.components.len() - 1).chain(
                vec![regex::Regex::new(r"(\..*)?$")
                    .unwrap()
                    .replace(
                        &self.components.iter().last().unwrap(),
                        replacement.as_str(),
                    )
                    .deref()]
                .into_iter(),
            ),
        )
    }
}

impl std::fmt::Display for FilePath {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "{}",
            self.components().collect::<Vec<_>>().join("/")
        )
    }
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(format!("{}", FilePath::new(&["foo"])), "foo");
        assert_eq!(format!("{}", FilePath::new(&["foo", "bar"])), "foo/bar");
    }

    #[test]
    fn with_extension() {
        assert_eq!(
            FilePath::new(&["foo"]).with_extension("c"),
            FilePath::new(&["foo.c"])
        );
        assert_eq!(
            FilePath::new(&["foo", "bar"]).with_extension("c"),
            FilePath::new(&["foo", "bar.c"])
        );
        assert_eq!(
            FilePath::new(&["foo.c"]).with_extension(""),
            FilePath::new(&["foo"])
        );
        assert_eq!(
            FilePath::new(&["foo.c"]).with_extension("h"),
            FilePath::new(&["foo.h"])
        );
    }
}
