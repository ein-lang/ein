use super::error::RepositoryError;
use super::output_repository::OutputRepository;
use std::path::PathBuf;

pub struct ModuleOutputRepository<'a> {
    output_repository: &'a OutputRepository,
    extension: &'static str,
}

impl<'a> ModuleOutputRepository<'a> {
    pub fn new(output_repository: &'a OutputRepository, extension: &'static str) -> Self {
        Self {
            output_repository,
            extension,
        }
    }

    pub fn load(
        &self,
        module_path: &sloth::ModulePath,
        vec: &mut Vec<u8>,
    ) -> Result<(), RepositoryError> {
        self.output_repository
            .load(self.resolve_module_path(module_path), vec)
    }

    pub fn store(
        &self,
        module_path: &sloth::ModulePath,
        data: &[u8],
    ) -> Result<(), RepositoryError> {
        self.output_repository
            .store(self.resolve_module_path(module_path), data)
    }

    fn resolve_module_path(&self, module_path: &sloth::ModulePath) -> PathBuf {
        let mut path = PathBuf::new();

        for component in module_path.components() {
            path.push(component);
        }

        path.with_extension(self.extension)
    }
}
