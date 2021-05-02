use super::{
    file_path_configuration::FILE_PATH_CONFIGURATION,
    package_initialization_configuration::{
        DEFAULT_SYSTEM_PACKAGE_CONFIGURATION, PACKAGE_INITIALIZATION_CONFIGURATION,
    },
};
use std::fs::create_dir_all;

pub fn init(directory: &str, is_library: bool) -> Result<(), Box<dyn std::error::Error>> {
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
    package_initializer.initialize(&create_target(directory, is_library)?)?;

    Ok(())
}

fn create_target(
    directory: impl AsRef<std::path::Path>,
    is_library: bool,
) -> Result<app::Target, Box<dyn std::error::Error>> {
    Ok(if is_library {
        app::Target::Library
    } else {
        app::Target::Application(app::ApplicationTarget::new(
            directory
                .as_ref()
                .canonicalize()?
                .file_name()
                .ok_or_else(|| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "cannot determine application name",
                    )
                })?
                .to_string_lossy(),
            DEFAULT_SYSTEM_PACKAGE_CONFIGURATION.clone(),
        ))
    })
}
