use super::file_path_configuration::FILE_PATH_CONFIGURATION;
use std::fs::create_dir_all;

pub fn init(target: &str, directory: &str) -> Result<(), Box<dyn std::error::Error>> {
    let os_file_path_converter = infra::OsFilePathConverter::new(directory);
    let file_system = infra::FileSystem::new(&os_file_path_converter);
    let project_initializer = app::ProjectInitializer::new(&file_system, &FILE_PATH_CONFIGURATION);

    create_dir_all(directory)?;
    project_initializer.initialize(&parse_target(target, directory)?)?;

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
        ))),
        "library" => Ok(app::Target::Library),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("target \"{}\" not supported", target),
        )
        .into()),
    }
}
