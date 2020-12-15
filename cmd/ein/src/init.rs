pub fn init(target: &str, directory: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file_path_converter = infra::FilePathConverter::new(directory);
    let file_storage = infra::FileStorage::new(&file_path_converter);
    let project_initializer = app::ProjectInitializer::new(&file_storage);

    project_initializer.initialize(parse_init_target(target)?)?;

    Ok(())
}

fn parse_init_target(target: &str) -> Result<app::InitTarget, std::io::Error> {
    match target {
        "command" => Ok(app::InitTarget::Command),
        "library" => Ok(app::InitTarget::Library),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("target \"{}\" not supported", target),
        )),
    }
}
