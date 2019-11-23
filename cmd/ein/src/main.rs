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
    let output_directory = std::path::Path::new(OUTPUT_DIRECTORY);

    let source_file_storage = infra::FileStorage::new(".", "ein");
    let object_file_storage = infra::FileStorage::new(output_directory.join("objects"), "bc");
    let interface_file_storage =
        infra::FileStorage::new(output_directory.join("interfaces"), "json");
    let relative_module_path_converter = app::RelativeModulePathConverter::new(&package);

    let module_compiler = app::ModuleCompiler::new(
        &relative_module_path_converter,
        &object_file_storage,
        &interface_file_storage,
    );
    let module_builder = app::ModuleBuilder::new(
        &module_compiler,
        &relative_module_path_converter,
        &source_file_storage,
    );

    app::PackageBuilder::new(
        &app::CommandPackageBuilder::new(
            &module_builder,
            &infra::Linker::new(std::env::var("EIN_ROOT")?, &object_file_storage),
        ),
        &app::LibraryPackageBuilder::new(
            &module_builder,
            &infra::Archiver::new(&object_file_storage, &interface_file_storage),
        ),
        &infra::ExternalPackageInitializer::new(
            &package_configuration,
            output_directory.join("packages"),
        ),
    )
    .build(&package_configuration.target().try_into()?)
}
