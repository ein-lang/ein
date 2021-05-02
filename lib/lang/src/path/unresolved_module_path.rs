use super::{
    external_unresolved_module_path::ExternalUnresolvedModulePath,
    internal_unresolved_module_path::InternalUnresolvedModulePath,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
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

impl std::fmt::Display for UnresolvedModulePath {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::External(external_module_path) => write!(formatter, "{}", external_module_path),
            Self::Internal(internal_module_path) => write!(formatter, "{}", internal_module_path),
        }
    }
}
