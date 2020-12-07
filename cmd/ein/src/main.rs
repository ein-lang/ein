const PACKAGE_CONFIGURATION_FILENAME: &str = "ein.json";

fn main() {
    if let Err(error) = run() {
        infra::Logger::new().log_error(error.as_ref()).unwrap();
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    match clap::App::new("ein")
        .version("0.1.0")
        .setting(clap::AppSettings::SubcommandRequired)
        .subcommand(clap::SubCommand::with_name("build"))
        .get_matches()
        .subcommand()
    {
        ("build", _) => build(),
        _ => unreachable!(),
    }
}

fn build() -> Result<(), Box<dyn std::error::Error>> {
    let package_directory = find_package_directory()?;

    let logger = infra::Logger::new();

    let file_path_configuration = app::FilePathConfiguration::new(
        PACKAGE_CONFIGURATION_FILENAME,
        ".ein",
        "package",
        "ein",
        "bc",
        "json",
    );

    let file_path_converter = infra::FilePathConverter::new(package_directory);
    let file_storage = infra::FileStorage::new(&file_path_converter);
    let file_path_manager = app::FilePathManager::new(&file_path_configuration);
    let file_path_displayer = infra::FilePathDisplayer::new(&file_path_converter);

    let command_runner = infra::CommandRunner::new();
    let module_objects_linker =
        infra::ModuleObjectsLinker::new(&command_runner, &file_path_converter);
    let module_parser = app::ModuleParser::new(&file_path_displayer);
    let compile_configuration = app::CompileConfiguration {
        source_main_function_name: "main".into(),
        object_main_function_name: "ein_main".into(),
        malloc_function_name: "ein_malloc".into(),
        panic_function_name: "ein_panic".into(),
        list_type_configuration: app::ListTypeConfiguration {
            empty_list_variable_name: "$emptyList".into(),
            concatenate_function_name: "$concatenateLists".into(),
            equal_function_name: "$equalLists".into(),
            prepend_function_name: "$prependToList".into(),
            deconstruct_function_name: "$firstRest".into(),
            first_function_name: "$first".into(),
            rest_function_name: "$rest".into(),
            list_type_name: "$AnyList".into(),
            first_rest_type_name: "$FirstRest".into(),
        }
        .into(),
        string_type_configuration: app::StringTypeConfiguration {
            equal_function_name: "ein_string_equal".into(),
        }
        .into(),
        system_type_configuration: app::SystemTypeConfiguration {
            system_type_name: "System".into(),
        }
        .into(),
    }
    .into();
    let module_compiler = app::ModuleCompiler::new(
        &module_parser,
        &file_path_manager,
        &file_storage,
        &logger,
        compile_configuration,
    );
    let modules_finder = app::ModulesFinder::new(&file_path_manager, &file_storage);
    let modules_builder = app::ModulesBuilder::new(
        &module_parser,
        &module_compiler,
        &modules_finder,
        &file_storage,
        &file_path_manager,
    );
    let module_interfaces_linker = app::ModuleInterfacesLinker::new(&file_storage);
    let modules_linker = app::ModulesLinker::new(
        &module_objects_linker,
        &module_interfaces_linker,
        &file_path_manager,
    );

    let package_configuration_reader = app::PackageConfigurationReader::new(
        &file_storage,
        &file_path_displayer,
        &file_path_configuration,
    );
    let package_builder = app::PackageBuilder::new(&modules_builder, &modules_linker, &logger);

    let root_directory_string = std::env::var("EIN_ROOT")?;
    let root_directory = std::path::Path::new(&root_directory_string);

    let prelude_package_downloader = infra::PreludePackageDownloader::new(
        &command_runner,
        &file_path_converter,
        root_directory.join("lib/prelude"),
    );
    let prelude_package_builder = app::PreludePackageBuilder::new(
        &package_configuration_reader,
        &package_builder,
        &prelude_package_downloader,
        &file_storage,
        &file_path_manager,
    );

    app::MainPackageBuilder::new(
        &package_configuration_reader,
        &package_builder,
        &infra::CommandLinker::new(
            &command_runner,
            &file_path_converter,
            root_directory.join("target/release/libruntime.a"),
        ),
        &prelude_package_builder,
        &app::ExternalPackagesDownloader::new(
            &package_configuration_reader,
            &infra::ExternalPackageDownloader::new(),
            &file_storage,
            &file_path_manager,
            &logger,
        ),
        &app::ExternalPackagesBuilder::new(&package_builder, &file_storage),
        &logger,
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
                    "file {} not found in any parent directory",
                    PACKAGE_CONFIGURATION_FILENAME
                ),
            )
        })?
    }

    Ok(directory.into())
}
