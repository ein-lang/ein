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
    let object_file_storage =
        infra::FileStorage::new(std::path::Path::new(OUTPUT_DIRECTORY).join("objects"), "bc");

    app::PackageBuilder::new(
        &app::ModuleCompiler::new(
            &app::RelativeModulePathConverter::new(&package),
            &infra::FileStorage::new(".", "ein"),
            &object_file_storage,
            &infra::FileStorage::new(
                std::path::Path::new(OUTPUT_DIRECTORY).join("interfaces"),
                "json",
            ),
        ),
        &infra::Linker::new(std::env::var("EIN_ROOT")?, &object_file_storage),
    )
    .build(&package_configuration.target().try_into()?)
}
