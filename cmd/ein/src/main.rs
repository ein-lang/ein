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
    let package_directory = find_package_directory()?;

    let file_path_configuration = app::FilePathConfiguration::new(
        PACKAGE_CONFIGURATION_FILENAME,
        "package",
        "ein",
        "bc",
        "json",
        app::FilePath::new(&[".ein"]),
    );

    let file_path_converter = infra::FilePathConverter::new(package_directory);
    let file_storage = infra::FileStorage::new(&file_path_converter);
    let file_path_manager = app::FilePathManager::new(&file_path_configuration);
    let file_path_displayer = infra::FilePathDisplayer::new(&file_path_converter);

    let object_linker = infra::ObjectLinker::new(&file_path_converter);
    let module_parser = app::ModuleParser::new(&file_path_displayer);
    let compile_configuration = app::CompileConfiguration::new("main", "ein_main", "ein_init");
    let module_compiler = app::ModuleCompiler::new(
        &module_parser,
        &file_path_manager,
        &file_storage,
        &compile_configuration,
    );
    let source_file_paths_finder =
        app::SourceFilePathsFinder::new(&file_path_manager, &file_storage);
    let modules_builder = app::ModulesBuilder::new(
        &module_parser,
        &module_compiler,
        &source_file_paths_finder,
        &file_storage,
        &file_path_manager,
    );
    let interface_linker = app::InterfaceLinker::new(&file_storage);
    let modules_linker =
        app::ModulesLinker::new(&object_linker, &interface_linker, &file_path_manager);
    let package_initializer = app::PackageInitializer::new(&file_storage, &file_path_configuration);
    let package_builder = app::PackageBuilder::new(&modules_builder, &modules_linker);
    let root_directory_string = std::env::var("EIN_ROOT")?;
    let root_directory = std::path::Path::new(&root_directory_string);

    app::MainPackageBuilder::new(
        &package_initializer,
        &package_builder,
        &infra::CommandLinker::new(
            &file_path_converter,
            root_directory.join("target/release/libruntime.a"),
        ),
        &app::ExternalPackagesDownloader::new(
            &package_initializer,
            &infra::ExternalPackageDownloader::new(),
            &file_storage,
            &file_path_manager,
        ),
        &app::ExternalPackagesBuilder::new(&package_builder, &file_storage),
    )
    .build()
}

fn find_package_directory() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
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

    Ok(directory.into())
}
