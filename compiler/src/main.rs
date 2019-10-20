extern crate clap;
extern crate core;
extern crate indoc;
extern crate nom;
extern crate serde;
extern crate serde_json;

mod ast;
mod compile;
mod debug;
mod parse;
mod types;

use compile::compile;
use parse::parse;

fn main() {
    let arguments = clap::App::new("Sloth Compiler")
        .version("0.1.0")
        .arg(
            clap::Arg::with_name("module_name")
                .short("m")
                .takes_value(true)
                .required(true),
        )
        .arg(
            clap::Arg::with_name("module_interface_directory")
                .short("i")
                .takes_value(true)
                .required(true),
        )
        .arg(
            clap::Arg::with_name("output_filename")
                .short("o")
                .takes_value(true)
                .required(true),
        )
        .arg(
            clap::Arg::with_name("input_filename")
                .index(1)
                .required(true),
        )
        .get_matches_safe()
        .unwrap_or_else(handle_error);

    let input_filename = arguments
        .value_of("input_filename")
        .ok_or_else(|| invalid_input_error("no input file"))
        .unwrap_or_else(handle_error);

    let module = parse(
        &std::fs::read_to_string(input_filename).unwrap_or_else(handle_error),
        input_filename,
    )
    .unwrap_or_else(handle_error);

    compile(
        arguments
            .value_of("module_name")
            .ok_or_else(|| invalid_input_error("no module name"))
            .unwrap_or_else(handle_error),
        &module,
        &module
            .imports()
            .iter()
            .map(|import| {
                serde_json::from_str(
                    &std::fs::read_to_string(resolve_absolute_module_path(
                        import.module_path(),
                        arguments
                            .value_of("input_filename")
                            .ok_or_else(|| invalid_input_error("no module interface directory"))
                            .unwrap_or_else(handle_error),
                    ))
                    .unwrap_or_else(handle_error),
                )
            })
            .collect::<Result<Vec<_>, _>>()
            .unwrap_or_else(handle_error),
        arguments
            .value_of("output_filename")
            .ok_or_else(|| invalid_input_error("no output file"))
            .unwrap_or_else(handle_error),
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

fn resolve_absolute_module_path(module_path: &ast::ModulePath, root_directory: &str) -> String {
    vec![root_directory]
        .iter()
        .map(|string| *string)
        .chain(
            match module_path {
                ast::ModulePath::External(_) => unimplemented!(),
                ast::ModulePath::Internal(path_elements) => path_elements.iter(),
            }
            .map(|string| string.as_str()),
        )
        .collect::<Vec<&str>>()
        .concat()
}
