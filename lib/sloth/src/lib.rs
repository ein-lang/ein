extern crate core;
extern crate indoc;
extern crate nom;
extern crate serde;

pub mod ast;
mod compile;
pub mod debug;
mod parse;
mod path;
pub mod types;

pub use compile::compile;
pub use parse::{parse_module, parse_module_path, Source};
pub use path::ModulePath;

pub fn serialize_module_interface(
    module_interface: &ast::ModuleInterface,
) -> Result<Vec<u8>, serde_json::Error> {
    Ok(serde_json::to_string(module_interface)?.as_bytes().into())
}

pub fn deserialize_module_interface(
    data: &[u8],
) -> Result<ast::ModuleInterface, serde_json::Error> {
    Ok(serde_json::from_slice(data)?)
}
