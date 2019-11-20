use super::module_compiler::ModuleCompiler;
use crate::infra::{Archiver, FileStorage};

pub struct LibraryPackageBuilder<'a, S: FileStorage, A: Archiver> {
    module_compiler: &'a ModuleCompiler<'a, S>,
    #[allow(dead_code)]
    archiver: &'a A,
    source_file_storage: &'a S,
}

impl<'a, S: FileStorage, A: Archiver> LibraryPackageBuilder<'a, S, A> {
    pub fn new(
        module_compiler: &'a ModuleCompiler<'a, S>,
        archiver: &'a A,
        source_file_storage: &'a S,
    ) -> Self {
        Self {
            module_compiler,
            archiver,
            source_file_storage,
        }
    }

    pub fn build(&self) -> Result<(), Box<dyn std::error::Error>> {
        let _ = self
            .source_file_storage
            .get_file_paths()?
            .iter()
            .map(|source_file_path| self.module_compiler.compile(source_file_path))
            .collect::<Result<Vec<_>, _>>()?;

        unimplemented!()
    }
}
