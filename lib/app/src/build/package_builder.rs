use super::command_package_builder::CommandPackageBuilder;
use super::library_package_builder::LibraryPackageBuilder;
use super::target::Target;
use crate::infra::{Archiver, ExternalPackageInitializer, FileStorage, Linker};

pub struct PackageBuilder<'a, S: FileStorage, L: Linker, A: Archiver, E: ExternalPackageInitializer>
{
    command_package_builder: &'a CommandPackageBuilder<'a, S, L>,
    library_package_builder: &'a LibraryPackageBuilder<'a, S, A>,
    external_package_initializer: &'a E,
}

impl<'a, S: FileStorage, L: Linker, A: Archiver, E: ExternalPackageInitializer>
    PackageBuilder<'a, S, L, A, E>
{
    pub fn new(
        command_package_builder: &'a CommandPackageBuilder<'a, S, L>,
        library_package_builder: &'a LibraryPackageBuilder<'a, S, A>,
        external_package_initializer: &'a E,
    ) -> Self {
        Self {
            command_package_builder,
            library_package_builder,
            external_package_initializer,
        }
    }

    pub fn build(&self, target: &Target) -> Result<(), Box<dyn std::error::Error>> {
        self.external_package_initializer.initialize()?;

        match target {
            Target::Command(command_target) => {
                self.command_package_builder.build(command_target.name())
            }
            Target::Library => self.library_package_builder.build(),
        }
    }
}
