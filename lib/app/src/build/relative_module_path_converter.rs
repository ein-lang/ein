use crate::infra::FilePath;

pub struct RelativeModulePathConverter<'a> {
    package: &'a ein::Package,
}

impl<'a> RelativeModulePathConverter<'a> {
    pub fn new(package: &'a ein::Package) -> Self {
        Self { package }
    }

    pub fn convert_from_file_path(&self, file_path: &FilePath) -> ein::ModulePath {
        ein::ModulePath::new(self.package.clone(), file_path.components().to_vec())
    }

    pub fn convert_to_file_path(
        &self,
        relative_module_path: &ein::RelativeUnresolvedModulePath,
    ) -> FilePath {
        FilePath::new(relative_module_path.components().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_from_file_path() {
        assert_eq!(
            RelativeModulePathConverter::new(&ein::Package::new("package", ""))
                .convert_from_file_path(&FilePath::new(vec!["Foo".into()])),
            ein::ModulePath::new(ein::Package::new("package", ""), vec!["Foo".into()])
        );
    }

    #[test]
    fn convert_to_file_path() {
        assert_eq!(
            RelativeModulePathConverter::new(&ein::Package::new("package", ""))
                .convert_to_file_path(
                    &ein::RelativeUnresolvedModulePath::new(vec!["Foo".into()]).into()
                ),
            FilePath::new(vec!["Foo".into()])
        );
    }
}
