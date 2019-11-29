use super::utilities;

#[derive(Default)]
pub struct FileStorage;

impl FileStorage {
    pub fn new() -> Self {
        Self
    }
}

impl app::FileStorage for FileStorage {
    fn exists(&self, file_path: &app::FilePath) -> bool {
        utilities::convert_to_os_path(file_path).exists()
    }

    fn glob(&self, pattern: &str) -> Result<Vec<app::FilePath>, Box<dyn std::error::Error>> {
        Ok(glob::glob(pattern)?
            .map(|path| {
                Ok(app::FilePath::new(
                    path?
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
            .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?
            .into_iter()
            .filter(|file_path| {
                file_path
                    .components()
                    .all(|component| !component.starts_with('.'))
            })
            .collect())
    }

    fn read_to_vec(
        &self,
        file_path: &app::FilePath,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(std::fs::read(utilities::convert_to_os_path(file_path))?)
    }

    fn read_to_string(
        &self,
        file_path: &app::FilePath,
    ) -> Result<String, Box<dyn std::error::Error>> {
        Ok(std::fs::read_to_string(utilities::convert_to_os_path(
            file_path,
        ))?)
    }

    fn write(
        &self,
        file_path: &app::FilePath,
        data: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let path = utilities::convert_to_os_path(file_path);

        if let Some(directory) = path.parent() {
            std::fs::create_dir_all(directory)?;
        }

        std::fs::write(path, data)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use app::FileStorage as FileStorageTrait;

    #[test]
    fn get_file_paths() {
        assert!(FileStorage::new()
            .glob("**/*.rs")
            .unwrap()
            .iter()
            .any(|file_path| file_path == &app::FilePath::new(&["src", "file_storage.rs"])));
    }
}
