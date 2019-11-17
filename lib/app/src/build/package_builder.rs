use super::module_compiler::ModuleCompiler;
use super::target::Target;
use crate::infra::{FilePath, FileStorage, Linker};

pub struct PackageBuilder<'a, S: FileStorage, L: Linker> {
    module_compiler: &'a ModuleCompiler<'a, S>,
    linker: &'a L,
}

impl<'a, S: FileStorage, L: Linker> PackageBuilder<'a, S, L> {
    pub fn new(module_compiler: &'a ModuleCompiler<'a, S>, linker: &'a L) -> Self {
        Self {
            module_compiler,
            linker,
        }
    }

    pub fn build(&self, target: &Target) -> Result<(), Box<dyn std::error::Error>> {
        let object_file_path = self
            .module_compiler
            .compile(&FilePath::new(vec!["Main".into()]))?;

        if let Target::Command(command_target) = target {
            self.linker.link(&object_file_path, command_target.name())?;
        }

        Ok(())
    }
}
