use super::package_interface::PackageInterface;
use crate::infra::{FilePath, FileSystem};

pub struct ModuleInterfacesLinker<'a> {
    file_system: &'a dyn FileSystem,
}

impl<'a> ModuleInterfacesLinker<'a> {
    pub fn new(file_system: &'a dyn FileSystem) -> Self {
        Self { file_system }
    }

    pub fn link<'b>(
        &self,
        interface_file_paths: impl IntoIterator<Item = &'b FilePath>,
        package_interface_file_path: &FilePath,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.file_system.write(
            package_interface_file_path,
            serde_json::to_string(&PackageInterface::new(
                &interface_file_paths
                    .into_iter()
                    .map(|file_path| {
                        Ok(serde_json::from_slice(
                            &self.file_system.read_to_vec(&file_path)?,
                        )?)
                    })
                    .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?,
            ))?
            .as_bytes(),
        )?;

        Ok(())
    }
}
