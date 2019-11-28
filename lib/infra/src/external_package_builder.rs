use super::command_runner::CommandRunner;

#[derive(Default)]
pub struct ExternalPackageBuilder;

impl ExternalPackageBuilder {
    pub fn new() -> Self {
        Self
    }
}

impl app::ExternalPackageBuilder for ExternalPackageBuilder {
    fn build(&self, directory_path: &app::FilePath) -> Result<(), Box<dyn std::error::Error>> {
        let directory_path = directory_path
            .components()
            .iter()
            .collect::<std::path::PathBuf>();

        CommandRunner::run(
            std::process::Command::new("ein")
                .arg("build")
                .current_dir(&directory_path),
        )?;

        tar::Archive::new(std::fs::File::open(directory_path.join("library.tar"))?)
            .unpack(&directory_path)?;

        Ok(())
    }
}
