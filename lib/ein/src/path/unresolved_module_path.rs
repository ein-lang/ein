use super::external_unresolved_module_path::ExternalUnresolvedModulePath;
use super::internal_unresolved_module_path::InternalUnresolvedModulePath;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum UnresolvedModulePath {
    External(ExternalUnresolvedModulePath),
    Internal(InternalUnresolvedModulePath),
}

impl From<ExternalUnresolvedModulePath> for UnresolvedModulePath {
    fn from(path: ExternalUnresolvedModulePath) -> Self {
        Self::External(path)
    }
}

impl From<InternalUnresolvedModulePath> for UnresolvedModulePath {
    fn from(path: InternalUnresolvedModulePath) -> Self {
        Self::Internal(path)
    }
}
