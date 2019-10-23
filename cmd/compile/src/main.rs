extern crate clap;
extern crate infra;
extern crate serde_json;
extern crate sloth;

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
            clap::Arg::with_name("source_directory")
                .short("s")
                .takes_value(true)
                .required(true),
        )
        .arg(
            clap::Arg::with_name("output_directory")
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

    let input_filename = arguments.value_of("input_filename").unwrap();
    let module_path =
        infra::ModulePathConverter::new(arguments.value_of("source_directory").unwrap())
            .convert_from_fs_path(input_filename)?;
    let module = sloth::parse_module(sloth::Source::new(
        input_filename,
        &std::fs::read_to_string(input_filename)?,
    ))?;

    let output_repository =
        infra::OutputRepository::new(arguments.value_of("output_directory").unwrap());
    let module_interface_repository = infra::ModuleInterfaceRepository::new(&output_repository);

    let (module_object, module_interface) = sloth::compile(&sloth::ast::Module::new(
        module_path.clone(),
        module.export().clone(),
        module
            .imports()
            .iter()
            .map(
                |import| -> Result<sloth::ast::ModuleInterface, Box<dyn std::error::Error>> {
                    Ok(module_interface_repository.load(import.module_path())?)
                },
            )
            .collect::<Result<Vec<_>, _>>()?,
        module.definitions().to_vec(),
    ))?;

    infra::ModuleObjectRepository::new(&output_repository).store(&module_path, &module_object)?;
    module_interface_repository.store(&module_path, &module_interface)?;

    Ok(())
}
