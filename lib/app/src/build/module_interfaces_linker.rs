use super::package_interface::PackageInterface;
use crate::infra::{FilePath, FileStorage};

pub struct ModuleInterfacesLinker<'a> {
    file_storage: &'a dyn FileStorage,
}

impl<'a> ModuleInterfacesLinker<'a> {
    pub fn new(file_storage: &'a dyn FileStorage) -> Self {
        Self { file_storage }
    }

    pub fn link<'b>(
        &self,
        interface_file_paths: impl IntoIterator<Item = &'b FilePath>,
        package_interface_file_path: &FilePath,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.file_storage.write(
            package_interface_file_path,
            serde_json::to_string(&PackageInterface::new(
                &interface_file_paths
                    .into_iter()
                    .map(|file_path| {
                        Ok(serde_json::from_slice(
                            &self.file_storage.read_to_vec(&file_path)?,
                        )?)
                    })
                    .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?,
            ))?
            .as_bytes(),
        )?;

        Ok(())
    }
}
