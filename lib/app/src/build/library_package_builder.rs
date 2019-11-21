use super::module_builder::ModuleBuilder;
use crate::infra::{Archiver, FileStorage};

pub struct LibraryPackageBuilder<'a, S: FileStorage, A: Archiver> {
    module_builder: &'a ModuleBuilder<'a, S>,
    #[allow(dead_code)]
    archiver: &'a A,
}

impl<'a, S: FileStorage, A: Archiver> LibraryPackageBuilder<'a, S, A> {
    pub fn new(module_builder: &'a ModuleBuilder<'a, S>, archiver: &'a A) -> Self {
        Self {
            module_builder,
            archiver,
        }
    }

    pub fn build(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.module_builder.build()?;

        unimplemented!()
    }
}
