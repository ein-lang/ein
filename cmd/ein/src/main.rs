use std::convert::TryInto;

const OUTPUT_DIRECTORY: &str = ".ein";

fn main() {
    if let Err(error) = run() {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    match clap::App::new("ein")
        .setting(clap::AppSettings::SubcommandRequired)
        .subcommand(clap::SubCommand::with_name("build"))
        .get_matches_safe()?
        .subcommand()
    {
        ("build", _) => build(),
        _ => unreachable!(),
    }
}

fn build() -> Result<(), Box<dyn std::error::Error>> {
    let package_configuration =
        infra::parse_package_configuration(&std::fs::read_to_string("ein.json")?)?;
    let package = infra::get_package()?;

    let source_file_storage = infra::FileStorage::new(".", "ein");
    let object_file_storage =
        infra::FileStorage::new(std::path::Path::new(OUTPUT_DIRECTORY).join("objects"), "bc");
    let interface_file_storage = infra::FileStorage::new(
        std::path::Path::new(OUTPUT_DIRECTORY).join("interfaces"),
        "json",
    );
    let relative_module_path_converter = app::RelativeModulePathConverter::new(&package);

    match package_configuration.target().try_into()? {
        app::Target::Command(command_target) => app::CommandPackageBuilder::new(
            &app::ModuleBuilder::new(
                &app::ModuleCompiler::new(
                    &relative_module_path_converter,
                    &object_file_storage,
                    &interface_file_storage,
                ),
                &relative_module_path_converter,
                &source_file_storage,
            ),
            &infra::Linker::new(std::env::var("EIN_ROOT")?, &object_file_storage),
        )
        .build(command_target.name()),
        app::Target::Library => app::LibraryPackageBuilder::new(
            &app::ModuleBuilder::new(
                &app::ModuleCompiler::new(
                    &relative_module_path_converter,
                    &object_file_storage,
                    &interface_file_storage,
                ),
                &relative_module_path_converter,
                &source_file_storage,
            ),
            &infra::Archiver::new(&object_file_storage, &interface_file_storage),
        )
        .build(),
    }
}
