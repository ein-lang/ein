pub struct FileStorage {
    directory: Box<std::path::Path>,
    extension: String,
}

impl FileStorage {
    pub fn new(directory: impl AsRef<std::path::Path>, extension: impl Into<String>) -> Self {
        Self {
            directory: directory.as_ref().into(),
            extension: extension.into(),
        }
    }

    pub fn resolve_file_path(&self, file_path: &app::FilePath) -> std::path::PathBuf {
        self.directory
            .join(
                file_path
                    .components()
                    .iter()
                    .collect::<std::path::PathBuf>(),
            )
            .with_extension(&self.extension)
    }
}

impl app::FileStorage for FileStorage {
    fn exists(&self, file_path: &app::FilePath) -> bool {
        self.resolve_file_path(file_path).exists()
    }

    fn get_file_paths(&self) -> Result<Vec<app::FilePath>, std::io::Error> {
        let directory = self.directory.canonicalize()?;

        Ok(glob::glob(
            &[
                &format!("{}", directory.display()),
                "/**/*",
                &self.extension,
            ]
            .concat(),
        )
        .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))?
        .map(|path| {
            Ok(app::FilePath::new(
                path.map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))?
                    .strip_prefix(&directory)
                    .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))?
                    .with_extension("")
                    .components()
                    .map(|component| match component {
                        std::path::Component::Normal(component) => {
                            Some(component.to_string_lossy().into())
                        }
                        _ => None,
                    })
                    .collect::<Option<Vec<String>>>()
                    .unwrap(),
            ))
        })
        .collect::<Result<Vec<app::FilePath>, std::io::Error>>()?
        .into_iter()
        .filter(|file_path| {
            file_path
                .components()
                .iter()
                .all(|component| !component.starts_with('.'))
        })
        .collect())
    }

    fn read_to_vec(&self, file_path: &app::FilePath) -> Result<Vec<u8>, std::io::Error> {
        std::fs::read(self.resolve_file_path(file_path))
    }

    fn read_to_string(&self, file_path: &app::FilePath) -> Result<String, std::io::Error> {
        std::fs::read_to_string(self.resolve_file_path(file_path))
    }

    fn write(&self, file_path: &app::FilePath, data: &[u8]) -> Result<(), std::io::Error> {
        let path = self.resolve_file_path(file_path);

        if let Some(directory) = path.parent() {
            std::fs::create_dir_all(directory)?;
        }

        std::fs::write(path, data)
    }
}

#[cfg(test)]
mod tests {
    use app::FileStorage;

    #[test]
    fn get_file_paths() {
        assert!(super::FileStorage::new(".", "rs")
            .get_file_paths()
            .unwrap()
            .iter()
            .any(|file_path| file_path
                == &app::FilePath::new(vec!["src".into(), "file_storage".into()])));
    }
}
