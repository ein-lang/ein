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
    let file_path_configuration = app::FilePathConfiguration::new(".ein", "ein", "bc", "json");

    let file_storage = infra::FileStorage::new();
    let internal_module_path_converter =
        app::InternalModulePathConverter::new(&file_path_configuration);
    let external_module_path_converter =
        app::ExternalModulePathConverter::new(&file_path_configuration);

    let module_compiler = app::ModuleCompiler::new(
        &internal_module_path_converter,
        &external_module_path_converter,
        &file_storage,
    );
    let module_builder = app::ModuleBuilder::new(
        &module_compiler,
        &file_storage,
        &internal_module_path_converter,
    );

    app::PackageBuilder::new(
        &app::CommandPackageBuilder::new(
            &module_builder,
            &infra::Linker::new(std::env::var("EIN_ROOT")?),
        ),
        &app::LibraryPackageBuilder::new(
            &module_builder,
            &infra::Archiver::new(),
            &internal_module_path_converter,
        ),
        &app::ExternalPackageInitializer::new(
            &infra::ExternalPackageDownloader::new(),
            &infra::ExternalPackageBuilder::new(),
            &external_module_path_converter,
            &file_storage,
        ),
        &infra::Repository::new(),
        &file_storage,
    )
    .build()
}
