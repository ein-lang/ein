use super::error::RepositoryError;
use super::module_output_repository::ModuleOutputRepository;
use super::output_repository::OutputRepository;

pub struct ModuleObjectRepository<'a> {
    module_output_repository: ModuleOutputRepository<'a>,
}

impl<'a> ModuleObjectRepository<'a> {
    pub fn new(output_repository: &'a OutputRepository) -> Self {
        Self {
            module_output_repository: ModuleOutputRepository::new(output_repository, "bc"),
        }
    }

    pub fn store(
        &self,
        module_path: &sloth::ModulePath,
        module_object: &sloth::ModuleObject,
    ) -> Result<(), RepositoryError> {
        self.module_output_repository
            .store(module_path, module_object.as_bytes())
    }
}
