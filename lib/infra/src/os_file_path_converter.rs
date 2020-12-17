pub struct OsFilePathConverter {
    base_directory: std::path::PathBuf,
}

impl OsFilePathConverter {
    pub fn new(base_directory: impl AsRef<std::path::Path>) -> Self {
        Self {
            base_directory: base_directory.as_ref().into(),
        }
    }

    pub fn convert_to_os_path(&self, path: &app::FilePath) -> std::path::PathBuf {
        if path.components().count() == 0 {
            self.base_directory.clone()
        } else {
            self.base_directory
                .join(path.components().collect::<std::path::PathBuf>())
        }
    }

    pub fn convert_to_file_path(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<app::FilePath, std::path::StripPrefixError> {
        Ok(app::FilePath::new(
            path.as_ref()
                .strip_prefix(&self.base_directory)?
                .components()
                .filter_map(|component| match component {
                    std::path::Component::Normal(component) => {
                        Some(component.to_string_lossy().into())
                    }
                    _ => None,
                })
                .collect::<Vec<String>>(),
        ))
    }
}
