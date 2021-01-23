use super::error::InfrastructureError;
use super::file_path_converter::FilePathConverter;

pub struct FileSystem<'a> {
    file_path_converter: &'a FilePathConverter,
}

impl<'a> FileSystem<'a> {
    pub fn new(file_path_converter: &'a FilePathConverter) -> Self {
        Self {
            file_path_converter,
        }
    }

    fn read_directory_with_raw_error(
        &self,
        file_path: &app::FilePath,
    ) -> Result<Vec<app::FilePath>, std::io::Error> {
        let path = self.file_path_converter.convert_to_os_path(file_path);

        path.read_dir()?
            .map(|entry| {
                Ok(self
                    .file_path_converter
                    .convert_absolute_to_file_path(entry?.path())
                    .unwrap())
            })
            .collect::<Result<_, std::io::Error>>()
    }

    fn read_repository_with_raw_error(
        &self,
        directory_path: &app::FilePath,
    ) -> Result<Option<app::Repository>, Box<dyn std::error::Error>> {
        let path = self.file_path_converter.convert_to_os_path(directory_path);

        if let Ok(repository) = git2::Repository::open(&path) {
            let url = if let Some(url) = repository
                .find_remote("origin")
                .ok()
                .and_then(|origin| origin.url().map(String::from))
            {
                url::Url::parse(&url)?
            } else {
                url::Url::from_directory_path(path).unwrap()
            };

            Ok(Some(app::Repository::new(
                url,
                format!("{}", repository.head()?.peel(git2::ObjectType::Any)?.id()),
            )))
        } else {
            Ok(None)
        }
    }
}

impl<'a> app::FileSystem for FileSystem<'a> {
    fn exists(&self, file_path: &app::FilePath) -> bool {
        self.file_path_converter
            .convert_to_os_path(file_path)
            .exists()
    }

    fn is_directory(&self, file_path: &app::FilePath) -> bool {
        self.file_path_converter
            .convert_to_os_path(file_path)
            .is_dir()
    }

    fn read_directory(
        &self,
        file_path: &app::FilePath,
    ) -> Result<Vec<app::FilePath>, Box<dyn std::error::Error>> {
        Ok(self
            .read_directory_with_raw_error(file_path)
            .map_err(|source| InfrastructureError::ReadDirectory {
                path: self.file_path_converter.convert_to_os_path(file_path),
                source,
            })?)
    }

    fn read_repository(
        &self,
        directory_path: &app::FilePath,
    ) -> Result<Option<app::Repository>, Box<dyn std::error::Error>> {
        Ok(self
            .read_repository_with_raw_error(directory_path)
            .map_err(|source| InfrastructureError::ReadRepository {
                path: self.file_path_converter.convert_to_os_path(directory_path),
                source,
            })?)
    }

    fn read_to_vec(
        &self,
        file_path: &app::FilePath,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let path = self.file_path_converter.convert_to_os_path(file_path);

        Ok(
            std::fs::read(&path)
                .map_err(|source| InfrastructureError::ReadFile { path, source })?,
        )
    }

    fn read_to_string(
        &self,
        file_path: &app::FilePath,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let path = self.file_path_converter.convert_to_os_path(file_path);

        Ok(std::fs::read_to_string(&path)
            .map_err(|source| InfrastructureError::ReadFile { path, source })?)
    }

    fn write(
        &self,
        file_path: &app::FilePath,
        data: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let path = self.file_path_converter.convert_to_os_path(file_path);

        if let Some(directory) = path.parent() {
            std::fs::create_dir_all(directory).map_err(|source| {
                InfrastructureError::CreateDirectory {
                    path: directory.into(),
                    source,
                }
            })?;
        }

        std::fs::write(&path, data)
            .map_err(|source| InfrastructureError::WriteFile { path, source })?;

        Ok(())
    }
}
