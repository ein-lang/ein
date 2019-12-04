use super::module_builder::ModuleBuilder;
use crate::infra::{FileStorage, Linker};

pub struct CommandPackageBuilder<'a, S: FileStorage, L: Linker> {
    module_builder: &'a ModuleBuilder<'a, S>,
    linker: &'a L,
}

impl<'a, S: FileStorage, L: Linker> CommandPackageBuilder<'a, S, L> {
    pub fn new(module_builder: &'a ModuleBuilder<'a, S>, linker: &'a L) -> Self {
        Self {
            module_builder,
            linker,
        }
    }

    pub fn build(
        &self,
        package: &ein::Package,
        command_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut file_paths = self.module_builder.build(package)?;

        self.linker.link(
            &file_paths
                .drain(..)
                .map(|(object_file_path, _)| object_file_path)
                .collect::<Vec<_>>(),
            command_name,
        )?;

        Ok(())
    }
}
