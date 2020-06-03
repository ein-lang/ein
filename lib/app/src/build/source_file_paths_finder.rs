use super::path::FilePathManager;
use crate::infra::{FilePath, FileStorage};

pub struct SourceFilePathsFinder<'a> {
    file_path_manager: &'a FilePathManager<'a>,
    file_storage: &'a dyn FileStorage,
}

impl<'a> SourceFilePathsFinder<'a> {
    pub fn new(
        file_path_manager: &'a FilePathManager<'a>,
        file_storage: &'a dyn FileStorage,
    ) -> Self {
        Self {
            file_path_manager,
            file_storage,
        }
    }

    pub fn find(
        &self,
        directory_path: &FilePath,
    ) -> Result<Vec<FilePath>, Box<dyn std::error::Error>> {
        let mut source_file_paths = vec![];

        for path in self.file_storage.read_directory(directory_path)? {
            if path
                .relative_to(&directory_path)
                .components()
                .next()
                .unwrap()
                .starts_with('.')
            {
            } else if self.file_storage.is_directory(&path) {
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
