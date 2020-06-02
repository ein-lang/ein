use super::error::InfrastructureError;
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

    fn is_directory(&self, file_path: &app::FilePath) -> bool {
        utilities::convert_to_os_path(file_path).is_dir()
    }

    fn read_directory(
        &self,
        file_path: &app::FilePath,
    ) -> Result<Vec<app::FilePath>, Box<dyn std::error::Error>> {
        Ok(utilities::convert_to_os_path(file_path)
            .read_dir()?
            .map(|entry| Ok(utilities::convert_to_file_path(entry?.path())))
            .collect::<Result<_, std::io::Error>>()?)
    }

    fn read_repository(
        &self,
        directory_path: &app::FilePath,
    ) -> Result<app::Repository, Box<dyn std::error::Error>> {
        let repository = git2::Repository::discover(utilities::convert_to_os_path(directory_path))?;
        let url = url::Url::parse(
            repository
                .find_remote("origin")?
                .url()
                .ok_or(InfrastructureError::OriginUrlNotFound)?,
        )?;

        Ok(app::Repository::new(
            url,
            format!("{}", repository.head()?.peel(git2::ObjectType::Any)?.id()),
        ))
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
