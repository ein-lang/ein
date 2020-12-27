pub struct FilePathConverter {
    base_directory: std::path::PathBuf,
}

impl FilePathConverter {
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

    pub fn convert_absolute_to_file_path(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<app::FilePath, Box<dyn std::error::Error>> {
        Ok(app::FilePath::new(
            path.as_ref()
                .strip_prefix(&self.base_directory)
                .map_err(|_| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!(
                            "path outside package directory: {}",
                            path.as_ref().to_string_lossy()
                        ),
                    )
                })?
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

    pub fn convert_relative_to_file_path(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<app::FilePath, Box<dyn std::error::Error>> {
        Ok(app::FilePath::new(
            path.as_ref()
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
