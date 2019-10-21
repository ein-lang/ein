use super::absolute_module_path::AbsoluteModulePath;
use super::relative_module_path::RelativeModulePath;

#[derive(Clone, Debug, PartialEq)]
pub enum ModulePath {
    Absolute(AbsoluteModulePath),
    Relative(RelativeModulePath),
}

impl From<AbsoluteModulePath> for ModulePath {
    fn from(absolute_module_path: AbsoluteModulePath) -> Self {
        Self::Absolute(absolute_module_path)
    }
}

impl From<RelativeModulePath> for ModulePath {
    fn from(relative_module_path: RelativeModulePath) -> Self {
        Self::Relative(relative_module_path)
    }
}
