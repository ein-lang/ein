use super::package_configuration::PackageConfiguration;
use crate::infra::FilePath;

pub struct ModulePathConverter<'a> {
    package_configuration: &'a PackageConfiguration,
}

impl<'a> ModulePathConverter<'a> {
    pub fn new(package_configuration: &'a PackageConfiguration) -> Self {
        Self {
            package_configuration,
        }
    }

    pub fn convert_from_file_path(&self, file_path: &FilePath) -> sloth::ModulePath {
        sloth::ModulePath::new(
            vec![self.package_configuration.name().into()]
                .into_iter()
                .chain(file_path.components().iter().cloned())
                .collect(),
        )
    }

    pub fn convert_to_file_path(&self, module_path: &sloth::ModulePath) -> FilePath {
        let mut iterator = module_path.components().iter();
        iterator.next();
        FilePath::new(iterator.cloned().collect())
    }
}
