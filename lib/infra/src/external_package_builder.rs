use super::command_runner::CommandRunner;
use super::utilities;

#[derive(Default)]
pub struct ExternalPackageBuilder;

impl ExternalPackageBuilder {
    pub fn new() -> Self {
        Self
    }
}

impl app::ExternalPackageBuilder for ExternalPackageBuilder {
    fn build(&self, directory_path: &app::FilePath) -> Result<(), Box<dyn std::error::Error>> {
        let directory_path = utilities::convert_to_os_path(directory_path);

        CommandRunner::run(
            std::process::Command::new("ein")
                .arg("build")
                .current_dir(&directory_path),
        )?;

        Ok(())
    }
}
