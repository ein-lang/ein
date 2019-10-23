use super::path_conversion_error::PathConversionError;
use std::path::Path;

pub struct ModulePathConverter {
    base_directory: Box<Path>,
}

impl ModulePathConverter {
    pub fn new(base_directory: &str) -> Self {
        Self {
            base_directory: Path::new(base_directory).into(),
        }
    }

    pub fn convert_from_fs_path(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<sloth::ModulePath, PathConversionError> {
        Ok(sloth::ModulePath::new(
            path.as_ref()
                .canonicalize()?
                .with_extension("")
                .strip_prefix(&self.base_directory.canonicalize()?)?
                .components()
                .into_iter()
                .map(|component| component.as_os_str().to_str().unwrap().into())
                .collect(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn convert_to_os_path() {
        assert_eq!(
            ModulePathConverter::new("src").convert_from_fs_path("src/lib.rs"),
            Ok(sloth::ModulePath::new(vec!["lib".into()]))
        );
    }
}
