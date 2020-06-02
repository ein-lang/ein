const PACKAGE_CONFIGURATION_FILENAME: &str = "ein.json";

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
    move_to_package_directory()?;

    let file_path_configuration = app::FilePathConfiguration::new(
        PACKAGE_CONFIGURATION_FILENAME,
        "package",
        "ein",
        "bc",
        "json",
        app::FilePath::new(&[".ein"]),
    );

    let file_storage = infra::FileStorage::new();
    let file_path_manager = app::FilePathManager::new(&file_path_configuration);
    let file_path_displayer = infra::FilePathDisplayer::new();

    let object_linker = infra::ObjectLinker::new();
    let module_parser = app::ModuleParser::new(&file_path_displayer);
    let compile_configuration = app::CompileConfiguration::new("main", "ein_main", "ein_init");
    let module_compiler = app::ModuleCompiler::new(
        &module_parser,
        &file_path_manager,
        &file_storage,
        &compile_configuration,
    );
    let module_builder = app::ModuleBuilder::new(
        &module_parser,
        &module_compiler,
        &file_storage,
        &file_path_manager,
    );
    let interface_linker = app::InterfaceLinker::new(&file_storage);
    let package_linker =
        app::PackageLinker::new(&object_linker, &interface_linker, &file_path_manager);

    app::PackageBuilder::new(
        &module_builder,
        &package_linker,
        &infra::CommandLinker::new(std::env::var("EIN_ROOT")?),
        &app::PackageInitializer::new(&file_storage, &file_path_configuration),
        &app::ExternalPackageInitializer::new(
            &infra::ExternalPackageDownloader::new(),
            &infra::ExternalPackageBuilder::new(),
            &file_storage,
            &file_path_manager,
        ),
    )
    .build()
}

fn move_to_package_directory() -> Result<(), Box<dyn std::error::Error>> {
    let mut directory: &std::path::Path = &std::env::current_dir()?;

    while !directory.join(PACKAGE_CONFIGURATION_FILENAME).exists() {
        directory = directory.parent().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!(
                    "{} file not found in any parent directory",
                    PACKAGE_CONFIGURATION_FILENAME
                ),
            )
        })?
    }

    std::env::set_current_dir(directory)?;

    Ok(())
}
