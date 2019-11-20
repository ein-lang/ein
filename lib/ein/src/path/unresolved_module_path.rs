use super::absolute_unresolved_module_path::AbsoluteUnresolvedModulePath;
use super::relative_unresolved_module_path::RelativeUnresolvedModulePath;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum UnresolvedModulePath {
    Absolute(AbsoluteUnresolvedModulePath),
    Relative(RelativeUnresolvedModulePath),
}

impl From<AbsoluteUnresolvedModulePath> for UnresolvedModulePath {
    fn from(path: AbsoluteUnresolvedModulePath) -> Self {
        Self::Absolute(path)
    }
}

impl From<RelativeUnresolvedModulePath> for UnresolvedModulePath {
    fn from(path: RelativeUnresolvedModulePath) -> Self {
        Self::Relative(path)
    }
}
