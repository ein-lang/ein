#[macro_use]
mod attempt;
mod error;
mod parse_ffi_package_interface;
mod parse_module;
mod parsers;
mod utilities;

pub use error::ParseError;
pub use parse_ffi_package_interface::parse_ffi_package_interface;
pub use parse_module::parse;
