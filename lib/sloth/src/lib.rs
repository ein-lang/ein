extern crate core;
extern crate indoc;
extern crate nom;
extern crate serde;
extern crate serde_json;

pub mod ast;
mod compile;
pub mod debug;
mod parse;
mod path;
pub mod types;

pub use compile::compile;
pub use parse::{parse_module, parse_module_path, Source};
pub use path::ModulePath;
