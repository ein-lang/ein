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
mod path;
mod types;

use ast::ModuleInterface;
use compile::compile;
use parse::{parse_module, parse_module_path};
use path::ModulePathResolver;

fn main() {
    if let Err(error) = run() {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = clap::App::new("Sloth Compiler")
        .version("0.1.0")
        .arg(
            clap::Arg::with_name("module_path")
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
        .get_matches_safe()?;

    let input_filename = arguments
        .value_of("input_filename")
        .expect("input filename");

    let module = parse_module(&std::fs::read_to_string(input_filename)?, input_filename)?;

    let module_path_resolver = ModulePathResolver::new(
        arguments
            .value_of("module_interface_directory")
            .expect("module interface directory"),
    );

    compile(
        &ast::Module::new(
            parse_module_path(
                arguments.value_of("module_path").expect("module path"),
                "<module path argument>",
            )?,
            module.export().clone(),
            module
                .imports()
                .iter()
                .map(
                    |import| -> Result<ModuleInterface, Box<dyn std::error::Error>> {
                        Ok(serde_json::from_str(&std::fs::read_to_string(
                            module_path_resolver.resolve_module_interface(import.module_path()),
                        )?)?)
                    },
                )
                .collect::<Result<Vec<_>, _>>()?,
            module.definitions().to_vec(),
        ),
        arguments
            .value_of("output_filename")
            .expect("output filename"),
    )?;

    Ok(())
}
