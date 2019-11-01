use crate::infra::FilePath;

pub struct ModulePathConverter<'a> {
    package: &'a sloth::Package,
}

impl<'a> ModulePathConverter<'a> {
    pub fn new(package: &'a sloth::Package) -> Self {
        Self { package }
    }

    pub fn convert_from_file_path(&self, file_path: &FilePath) -> sloth::ModulePath {
        sloth::ModulePath::new(
            vec![self.package.name().into()]
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
