use crate::ast;
use crate::path::ModulePath;
use crate::types::Type;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ModuleInterface {
    path: ModulePath,
    types: HashMap<String, Type>,
}

impl From<&ast::ModuleInterface> for ModuleInterface {
    fn from(module_interface: &ast::ModuleInterface) -> Self {
        Self {
            path: module_interface.path().clone(),
            types: module_interface.types().clone(),
        }
    }
}

impl Into<ast::ModuleInterface> for ModuleInterface {
    fn into(self: ModuleInterface) -> ast::ModuleInterface {
        ast::ModuleInterface::new(self.path, self.types)
    }
}
