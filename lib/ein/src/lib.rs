#[macro_use]
extern crate lazy_static;

pub mod ast;
mod compile;
pub mod debug;
mod package;
mod parse;
mod path;
mod serialize;
pub mod types;

pub use compile::{compile, ModuleObject};
pub use package::Package;
pub use parse::parse_module;
pub use path::{
    ExternalUnresolvedModulePath, InternalUnresolvedModulePath, ModulePath, UnresolvedModulePath,
};
pub use serialize::{deserialize_module_interface, serialize_module_interface};
