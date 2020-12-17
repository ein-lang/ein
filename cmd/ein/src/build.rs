use super::compile_configuration::COMPILE_CONFIGURATION;
use super::file_path_configuration::{FILE_PATH_CONFIGURATION, PACKAGE_CONFIGURATION_FILENAME};

pub fn build() -> Result<(), Box<dyn std::error::Error>> {
    let package_directory = find_package_directory()?;

    let logger = infra::Logger::new();

    let os_file_path_converter = infra::OsFilePathConverter::new(package_directory);
    let file_system = infra::FileSystem::new(&os_file_path_converter);
    let file_path_manager = app::FilePathManager::new(&FILE_PATH_CONFIGURATION);
    let file_path_displayer = infra::FilePathDisplayer::new(&os_file_path_converter);

    let command_runner = infra::CommandRunner::new();
    let module_objects_linker =
        infra::ModuleObjectsLinker::new(&command_runner, &os_file_path_converter);
    let module_parser = app::ModuleParser::new(&file_path_displayer);
    let module_compiler = app::ModuleCompiler::new(
        &module_parser,
        &file_path_manager,
        &file_system,
        &logger,
        COMPILE_CONFIGURATION.clone(),
    );
    let modules_finder = app::ModulesFinder::new(&file_path_manager, &file_system);
    let modules_builder = app::ModulesBuilder::new(
        &module_parser,
        &module_compiler,
        &modules_finder,
        &file_system,
        &file_path_manager,
    );
    let module_interfaces_linker = app::ModuleInterfacesLinker::new(&file_system);
    let modules_linker = app::ModulesLinker::new(
        &module_objects_linker,
        &module_interfaces_linker,
        &file_path_manager,
    );

    let package_configuration_reader = app::PackageConfigurationReader::new(
        &file_system,
        &file_path_displayer,
        &FILE_PATH_CONFIGURATION,
    );
    let package_builder = app::PackageBuilder::new(&modules_builder, &modules_linker, &logger);

    let root_directory_string = std::env::var("EIN_ROOT")?;
    let root_directory = std::path::Path::new(&root_directory_string);

    let prelude_package_downloader = infra::PreludePackageDownloader::new(
        &command_runner,
        &os_file_path_converter,
        root_directory.join("lib/prelude"),
    );
    let prelude_package_builder = app::PreludePackageBuilder::new(
        &package_configuration_reader,
        &package_builder,
        &prelude_package_downloader,
        &file_system,
        &file_path_manager,
    );
    let command_linker = infra::CommandLinker::new(
        &command_runner,
        &os_file_path_converter,
        root_directory.join("target/release/libruntime.a"),
    );
    let external_package_downloader = infra::ExternalPackageDownloader::new();
    let external_packages_downloader = app::ExternalPackagesDownloader::new(
        &package_configuration_reader,
        &external_package_downloader,
        &file_system,
        &file_path_manager,
        &logger,
    );
    let external_packages_builder =
        app::ExternalPackagesBuilder::new(&package_builder, &file_system);
    let main_package_builder = app::MainPackageBuilder::new(
        &package_configuration_reader,
        &package_builder,
        &command_linker,
        &prelude_package_builder,
        &external_packages_downloader,
        &external_packages_builder,
        &logger,
    );

    main_package_builder.build()
}

fn find_package_directory() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let mut directory: &std::path::Path = &std::env::current_dir()?;

    while !directory.join(PACKAGE_CONFIGURATION_FILENAME).exists() {
        directory = directory.parent().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!(
                    "file {} not found in any parent directory",
                    PACKAGE_CONFIGURATION_FILENAME,
                ),
            )
        })?
    }

    Ok(directory.into())
}
