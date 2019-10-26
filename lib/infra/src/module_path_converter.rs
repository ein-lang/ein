use super::infrastructure_error::InfrastructureError;
use super::package_configuration::PackageConfiguration;
use std::path::{Path, PathBuf};

pub struct ModulePathConverter<'a> {
    package_configuration: &'a PackageConfiguration,
}

impl<'a> ModulePathConverter<'a> {
    pub fn new(package_configuration: &'a PackageConfiguration) -> Self {
        Self {
            package_configuration,
        }
    }

    pub fn convert_from_source_path(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<sloth::ModulePath, InfrastructureError> {
        Ok(sloth::ModulePath::new(
            vec![self.package_configuration.name().into()]
                .into_iter()
                .chain(
                    path.as_ref()
                        .canonicalize()?
                        .with_extension("")
                        .strip_prefix(self.package_configuration.source_directory())?
                        .components()
                        .into_iter()
                        .map(|component| component.as_os_str().to_str().unwrap().into()),
                )
                .collect(),
        ))
    }

    pub fn convert_to_interface_path(&self, module_path: &sloth::ModulePath) -> PathBuf {
        let mut path = PathBuf::new();

        for component in module_path.components() {
            path.push(component);
        }

        path
    }
}
