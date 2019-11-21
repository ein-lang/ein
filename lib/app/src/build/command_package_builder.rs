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

    pub fn build(&self, command_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.linker
            .link(&self.module_builder.build()?, command_name)?;

        Ok(())
    }
}
