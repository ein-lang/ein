use super::module_builder::ModuleBuilder;
use crate::infra::{CommandLinker, FileStorage, ObjectLinker};

pub struct CommandPackageBuilder<'a, S: FileStorage, OL: ObjectLinker, CL: CommandLinker> {
    module_builder: &'a ModuleBuilder<'a, S, OL>,
    command_linker: &'a CL,
}

impl<'a, S: FileStorage, OL: ObjectLinker, CL: CommandLinker> CommandPackageBuilder<'a, S, OL, CL> {
    pub fn new(module_builder: &'a ModuleBuilder<'a, S, OL>, command_linker: &'a CL) -> Self {
        Self {
            module_builder,
            command_linker,
        }
    }

    pub fn build(
        &self,
        package: &ein::Package,
        command_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (object_file_path, _) = self.module_builder.build(package)?;

        self.command_linker.link(&object_file_path, command_name)?;

        Ok(())
    }
}
