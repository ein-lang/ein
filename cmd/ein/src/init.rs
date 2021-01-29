use super::file_path_configuration::FILE_PATH_CONFIGURATION;
use super::package_initialization_configuration::{
    DEFAULT_SYSTEM_PACKAGE_CONFIGURATION, PACKAGE_INITIALIZATION_CONFIGURATION,
};
use std::fs::create_dir_all;

pub fn init(target: &str, directory: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file_path_converter = infra::FilePathConverter::new(directory);
    let file_system = infra::FileSystem::new(&file_path_converter);
    let static_file_path_manager = app::StaticFilePathManager::new(&FILE_PATH_CONFIGURATION);
    let package_initializer = app::PackageInitializer::new(
        &file_system,
        &static_file_path_manager,
        &FILE_PATH_CONFIGURATION,
        &PACKAGE_INITIALIZATION_CONFIGURATION,
    );

    create_dir_all(directory)?;
    package_initializer.initialize(&parse_target(target, directory)?)?;

    Ok(())
}

fn parse_target(
    target: &str,
    directory: impl AsRef<std::path::Path>,
) -> Result<app::Target, Box<dyn std::error::Error>> {
    match target {
        "command" => Ok(app::Target::Command(app::CommandTarget::new(
            directory
                .as_ref()
                .canonicalize()?
                .file_name()
                .ok_or_else(|| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "cannot determine command name",
                    )
                })?
                .to_string_lossy(),
            DEFAULT_SYSTEM_PACKAGE_CONFIGURATION.clone(),
        ))),
        "library" => Ok(app::Target::Library),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("target \"{}\" not supported", target),
        )
        .into()),
    }
}
