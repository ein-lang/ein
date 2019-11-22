use super::command_package_builder::CommandPackageBuilder;
use super::library_package_builder::LibraryPackageBuilder;
use super::target::Target;
use crate::infra::{Archiver, FileStorage, Linker};

pub struct PackageBuilder<'a, S: FileStorage, L: Linker, A: Archiver> {
    command_package_builder: &'a CommandPackageBuilder<'a, S, L>,
    library_package_builder: &'a LibraryPackageBuilder<'a, S, A>,
}

impl<'a, S: FileStorage, L: Linker, A: Archiver> PackageBuilder<'a, S, L, A> {
    pub fn new(
        command_package_builder: &'a CommandPackageBuilder<'a, S, L>,
        library_package_builder: &'a LibraryPackageBuilder<'a, S, A>,
    ) -> Self {
        Self {
            command_package_builder,
            library_package_builder,
        }
    }

    pub fn build(&self, target: &Target) -> Result<(), Box<dyn std::error::Error>> {
        match target {
            Target::Command(command_target) => {
                self.command_package_builder.build(command_target.name())
            }
            Target::Library => self.library_package_builder.build(),
        }
    }
}
