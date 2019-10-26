extern crate clap;
extern crate infra;
extern crate sloth;

use std::io::Write;

fn main() {
    if let Err(error) = run() {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = clap::App::new("Sloth Compiler")
        .version("0.1.0")
        .setting(clap::AppSettings::TrailingVarArg)
        .arg(
            clap::Arg::with_name("package_filename")
                .short("p")
                .takes_value(true)
                .required(true),
        )
        .arg(
            clap::Arg::with_name("object_filename")
                .short("o")
                .takes_value(true)
                .required(true),
        )
        .arg(
            clap::Arg::with_name("exported_interface_filename")
                .short("e")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("source_filename")
                .index(1)
                .required(true),
        )
        .arg(
            clap::Arg::with_name("imported_interface_filename")
                .index(2)
                .takes_value(true)
                .multiple(true),
        )
        .get_matches_safe()?;

    let source_filename = arguments.value_of("source_filename").unwrap();
    let module_path = infra::ModulePathConverter::new(&infra::PackageConfiguration::read(
        arguments.value_of("package_filename").unwrap(),
    )?)
    .convert_from_source_path(source_filename)?;
    let module = sloth::parse_module(sloth::Source::new(
        source_filename,
        &std::fs::read_to_string(source_filename)?,
    ))?;

    let imported_module_interfaces = arguments
        .values_of("imported_interface_filename")
        .unwrap_or(Default::default())
        .map(|imported_interface_filename| {
            Ok(sloth::deserialize_module_interface(&std::fs::read(
                imported_interface_filename,
            )?)?)
        })
        .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;

    let (module_object, module_interface) = sloth::compile(&sloth::ast::Module::new(
        module_path.clone(),
        module.export().clone(),
        module
            .imports()
            .iter()
            .map(
                |import| -> Result<sloth::ast::ModuleInterface, CompileError> {
                    for module_interface in &imported_module_interfaces {
                        if module_interface.path() == import.module_path() {
                            return Ok(module_interface.clone());
                        }
                    }

                    Err(CompileError::ModuleInterfaceNotFound(
                        import.module_path().clone(),
                    ))
                },
            )
            .collect::<Result<Vec<_>, _>>()?,
        module.definitions().to_vec(),
    ))?;

    std::fs::File::create(arguments.value_of("object_filename").unwrap())?
        .write_all(module_object.as_bytes())?;

    if let Some(exported_interface_filename) = arguments.value_of("exported_interface_filename") {
        std::fs::File::create(exported_interface_filename)?
            .write_all(&sloth::serialize_module_interface(&module_interface)?)?;
    }

    Ok(())
}

#[derive(Debug)]
enum CompileError {
    ModuleInterfaceNotFound(sloth::ModulePath),
}

impl std::error::Error for CompileError {}

impl std::fmt::Display for CompileError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CompileError::ModuleInterfaceNotFound(module_path) => {
                write!(formatter, "module interface {} not found", module_path)
            }
        }
    }
}
