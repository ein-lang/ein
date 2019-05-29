#[macro_use]
extern crate lalrpop_util;
extern crate llvm_sys;

mod ast;
mod compile;
mod parse;

use compile::compile;
use parse::parse;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut buffer = String::with_capacity(1024);

    std::io::stdin().read_to_string(&mut buffer)?;

    compile(&parse(&buffer).map_err(map_error)?).map_err(map_error)
}

fn map_error<E: std::error::Error>(error: E) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidInput, error.description())
}
