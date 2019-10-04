extern crate core;
extern crate indoc;
extern crate nom;

mod ast;
mod compile;
mod debug;
mod parse;
mod types;

use compile::compile;
use parse::parse;

fn main() {
    let arguments = std::env::args().collect::<Vec<String>>();

    let input_filename = arguments
        .get(1)
        .ok_or_else(|| invalid_input_error("no input file"))
        .unwrap_or_else(handle_error);

    let output_filename = arguments
        .get(2)
        .ok_or_else(|| invalid_input_error("no output file"))
        .unwrap_or_else(handle_error);

    compile(
        &parse(
            &std::fs::read_to_string(input_filename).unwrap_or_else(handle_error),
            input_filename,
        )
        .unwrap_or_else(handle_error),
        output_filename,
    )
    .unwrap_or_else(handle_error);
}

fn invalid_input_error(description: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidInput, description)
}

fn handle_error<T, E: std::error::Error + std::fmt::Display>(error: E) -> T {
    eprintln!("{}", error);
    std::process::exit(1);
}
