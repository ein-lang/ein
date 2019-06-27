extern crate core;
extern crate indoc;
extern crate nom;

mod ast;
mod compile;
mod debug;
mod environment;
mod parse;
mod types;

use compile::{compile, CompileOptions};
use parse::parse;

fn main() -> std::io::Result<()> {
    compile(
        &parse(&std::fs::read_to_string(
            std::env::args()
                .collect::<Vec<String>>()
                .get(1)
                .ok_or(invalid_input_error("no input file"))?,
        )?)
        .map_err(map_error)?,
        CompileOptions {
            root_directory: environment::root_directory().map_err(map_error)?,
        },
    )
    .map_err(map_error)
}

fn map_error<E: std::error::Error>(error: E) -> std::io::Error {
    invalid_input_error(error.description())
}

fn invalid_input_error(description: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidInput, description)
}
