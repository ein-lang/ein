use crate::infra::{FilePath, FilePathDisplayer};

pub struct ModuleParser<'a, D: FilePathDisplayer> {
    file_path_displayer: &'a D,
}

impl<'a, D: FilePathDisplayer> ModuleParser<'a, D> {
    pub fn new(file_path_displayer: &'a D) -> Self {
        Self {
            file_path_displayer,
        }
    }

    pub fn parse(
        &self,
        source: &str,
        file_path: &FilePath,
    ) -> Result<ein::UnresolvedModule, ein::ParseError> {
        ein::parse_module(source, &self.file_path_displayer.display(file_path))
    }
}
