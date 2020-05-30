use crate::path::UnresolvedModulePath;

#[derive(Clone, Debug, PartialEq)]
pub struct UnresolvedImport {
    module_path: UnresolvedModulePath,
}

impl UnresolvedImport {
    pub fn new(module_path: impl Into<UnresolvedModulePath>) -> Self {
        Self {
            module_path: module_path.into(),
        }
    }

    pub fn module_path(&self) -> &UnresolvedModulePath {
        &self.module_path
    }
}
