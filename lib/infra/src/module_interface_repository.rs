use super::error::RepositoryError;
use super::module_product_repository::ModuleProductRepository;
use super::product_repository::ProductRepository;

pub struct ModuleInterfaceRepository<'a> {
    module_product_repository: ModuleProductRepository<'a>,
}

impl<'a> ModuleInterfaceRepository<'a> {
    pub fn new(product_repository: &'a ProductRepository) -> Self {
        Self {
            module_product_repository: ModuleProductRepository::new(product_repository, "json"),
        }
    }

    pub fn load(
        &self,
        module_path: &sloth::ModulePath,
    ) -> Result<sloth::ast::ModuleInterface, RepositoryError> {
        let mut vec = Vec::new();

        self.module_product_repository.load(module_path, &mut vec)?;

        Ok(serde_json::from_slice(&vec)?)
    }

    pub fn store(
        &self,
        module_path: &sloth::ModulePath,
        module_interface: &sloth::ast::ModuleInterface,
    ) -> Result<(), RepositoryError> {
        self.module_product_repository
            .store(module_path, &serde_json::to_vec(module_interface)?)
    }
}
