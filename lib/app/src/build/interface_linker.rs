use super::package_interface::PackageInterface;
use crate::infra::{FilePath, FileStorage};

pub struct InterfaceLinker<'a, S: FileStorage> {
    file_storage: &'a S,
}

impl<'a, S: FileStorage> InterfaceLinker<'a, S> {
    pub fn new(file_storage: &'a S) -> Self {
        Self { file_storage }
    }

    pub fn link(
        &self,
        interface_file_paths: &[FilePath],
        package_interface_file_path: &FilePath,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let package_interface = PackageInterface::new(
            &interface_file_paths
                .iter()
                .map(|file_path| {
                    Ok(ein::deserialize_module_interface(
                        &self.file_storage.read_to_vec(&file_path)?,
                    )?)
                })
                .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?,
        );

        self.file_storage.write(
            package_interface_file_path,
            serde_json::to_string(&package_interface)?.as_bytes(),
        )?;

        Ok(())
    }
}
