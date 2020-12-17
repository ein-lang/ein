use super::path::FilePathManager;
use crate::infra::{FilePath, FileSystem};

pub struct ModulesFinder<'a> {
    file_path_manager: &'a FilePathManager<'a>,
    file_system: &'a dyn FileSystem,
}

impl<'a> ModulesFinder<'a> {
    pub fn new(
        file_path_manager: &'a FilePathManager<'a>,
        file_system: &'a dyn FileSystem,
    ) -> Self {
        Self {
            file_path_manager,
            file_system,
        }
    }

    pub fn find(
        &self,
        directory_path: &FilePath,
    ) -> Result<Vec<FilePath>, Box<dyn std::error::Error>> {
        let mut source_file_paths = vec![];

        for path in self.file_system.read_directory(directory_path)? {
            if path
                .relative_to(&directory_path)
                .components()
                .next()
                .unwrap()
                .starts_with('.')
            {
            } else if self.file_system.is_directory(&path) {
                source_file_paths.extend(self.find(&path)?);
            } else if path.has_extension(
                self.file_path_manager
                    .configuration()
                    .source_file_extension(),
            ) {
                source_file_paths.push(path);
            }
        }

        Ok(source_file_paths)
    }
}
