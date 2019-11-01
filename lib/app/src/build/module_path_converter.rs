use crate::infra::FilePath;

pub struct ModulePathConverter<'a> {
    package: &'a sloth::Package,
}

impl<'a> ModulePathConverter<'a> {
    pub fn new(package: &'a sloth::Package) -> Self {
        Self { package }
    }

    pub fn convert_from_file_path(&self, file_path: &FilePath) -> sloth::ModulePath {
        sloth::ModulePath::new(self.package.clone(), file_path.components().to_vec())
    }

    pub fn convert_to_file_path(&self, module_path: &sloth::UnresolvedModulePath) -> FilePath {
        FilePath::new(module_path.components().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_from_file_path() {
        assert_eq!(
            ModulePathConverter::new(&sloth::Package::new("M", (0, 0, 0)))
                .convert_from_file_path(&FilePath::new(vec!["foo".into()])),
            sloth::ModulePath::new(sloth::Package::new("M", (0, 0, 0)), vec!["foo".into()])
        );
    }

    #[test]
    fn convert_to_file_path() {
        assert_eq!(
            ModulePathConverter::new(&sloth::Package::new("M", (0, 0, 0)))
                .convert_to_file_path(&sloth::UnresolvedModulePath::new("M", vec!["foo".into()])),
            FilePath::new(vec!["foo".into()])
        );
    }
}
