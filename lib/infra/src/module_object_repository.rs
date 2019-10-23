use super::error::RepositoryError;
use super::module_product_repository::ModuleProductRepository;
use super::product_repository::ProductRepository;

pub struct ModuleObjectRepository<'a> {
    module_product_repository: ModuleProductRepository<'a>,
}

impl<'a> ModuleObjectRepository<'a> {
    pub fn new(product_repository: &'a ProductRepository) -> Self {
        Self {
            module_product_repository: ModuleProductRepository::new(product_repository, "bc"),
        }
    }

    pub fn store(
        &self,
        module_path: &sloth::ModulePath,
        module_object: &sloth::ModuleObject,
    ) -> Result<(), RepositoryError> {
        self.module_product_repository
            .store(module_path, module_object.as_bytes())
    }
}
