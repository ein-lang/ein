use super::error::RepositoryError;
use super::product_repository::ProductRepository;
use std::path::PathBuf;

pub struct ModuleProductRepository<'a> {
    product_repository: &'a ProductRepository,
    extension: &'static str,
}

impl<'a> ModuleProductRepository<'a> {
    pub fn new(product_repository: &'a ProductRepository, extension: &'static str) -> Self {
        Self {
            product_repository,
            extension,
        }
    }

    pub fn load(
        &self,
        module_path: &sloth::ModulePath,
        vec: &mut Vec<u8>,
    ) -> Result<(), RepositoryError> {
        self.product_repository
            .load(self.resolve_module_path(module_path), vec)
    }

    pub fn store(
        &self,
        module_path: &sloth::ModulePath,
        data: &[u8],
    ) -> Result<(), RepositoryError> {
        self.product_repository
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
