use super::error::CompileError;
use crate::ast::ModuleInterface;

#[derive(Debug)]
pub struct ModuleInterfaceBlob {
    json: String,
}

impl ModuleInterfaceBlob {
    pub fn new(module_interface: &ModuleInterface) -> Result<Self, CompileError> {
        Ok(Self {
            json: serde_json::to_string(module_interface)?,
        })
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.json.as_bytes()
    }
}
