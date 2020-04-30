use crate::ast::*;

pub fn serialize_module_interface(
    module_interface: &ModuleInterface,
) -> Result<Vec<u8>, serde_json::Error> {
    Ok(serde_json::to_string(module_interface)?.as_bytes().into())
}

pub fn deserialize_module_interface(data: &[u8]) -> Result<ModuleInterface, serde_json::Error> {
    serde_json::from_slice(data)
}
