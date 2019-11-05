use super::module_compiler::ModuleCompiler;
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

    pub fn build(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.linker.link(
            &self
                .module_compiler
                .compile(&FilePath::new(vec!["Main".into()]))?,
        )?;

        Ok(())
    }
}
