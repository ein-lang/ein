use super::module_compiler::ModuleCompiler;
use crate::infra::{FilePath, FileStorage, Linker};

pub struct CommandPackageBuilder<'a, S: FileStorage, L: Linker> {
    module_compiler: &'a ModuleCompiler<'a, S>,
    linker: &'a L,
}

impl<'a, S: FileStorage, L: Linker> CommandPackageBuilder<'a, S, L> {
    pub fn new(module_compiler: &'a ModuleCompiler<'a, S>, linker: &'a L) -> Self {
        Self {
            module_compiler,
            linker,
        }
    }

    pub fn build(&self, command_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let object_file_path = self
            .module_compiler
            .compile(&FilePath::new(vec!["Main".into()]))?;

        self.linker.link(&object_file_path, command_name)?;

        Ok(())
    }
}
