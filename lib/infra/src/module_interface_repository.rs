use super::module_output_repository::ModuleOutputRepository;
use super::output_repository::OutputRepository;
use super::repository_error::RepositoryError;

pub struct ModuleInterfaceRepository<'a> {
    module_output_repository: ModuleOutputRepository<'a>,
}

impl<'a> ModuleInterfaceRepository<'a> {
    pub fn new(output_repository: &'a OutputRepository) -> Self {
        Self {
            module_output_repository: ModuleOutputRepository::new(output_repository, "json"),
        }
    }

    pub fn load(
        &self,
        module_path: &sloth::ModulePath,
    ) -> Result<sloth::ast::ModuleInterface, RepositoryError> {
        let mut vec = Vec::new();

        self.module_output_repository.load(module_path, &mut vec)?;

        Ok(serde_json::from_slice(&vec)?)
    }

    pub fn store(
        &self,
        module_path: &sloth::ModulePath,
        module_interface: &sloth::ast::ModuleInterface,
    ) -> Result<(), RepositoryError> {
        self.module_output_repository
            .store(module_path, &serde_json::to_vec(module_interface)?)
    }
}
