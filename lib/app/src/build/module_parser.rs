use crate::{common::FilePath, infra::FilePathDisplayer};

pub struct ModuleParser<'a> {
    file_path_displayer: &'a dyn FilePathDisplayer,
}

impl<'a> ModuleParser<'a> {
    pub fn new(file_path_displayer: &'a dyn FilePathDisplayer) -> Self {
        Self {
            file_path_displayer,
        }
    }

    pub fn parse(
        &self,
        source: &str,
        file_path: &FilePath,
    ) -> Result<lang::UnresolvedModule, lang::ParseError> {
        lang::parse(source, &self.file_path_displayer.display(file_path))
    }
}
