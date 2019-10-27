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
