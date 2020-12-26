use super::command_runner::CommandRunner;
use super::file_path_converter::FilePathConverter;

pub struct FfiPackageInitializer<'a> {
    command_runner: &'a CommandRunner,
    file_path_converter: &'a FilePathConverter,
}

impl<'a> FfiPackageInitializer<'a> {
    pub fn new(
        command_runner: &'a CommandRunner,
        file_path_converter: &'a FilePathConverter,
    ) -> Self {
        Self {
            command_runner,
            file_path_converter,
        }
    }
}

impl<'a> app::FfiPackageInitializer for FfiPackageInitializer<'a> {
    fn initialize(&self, directory_path: &app::FilePath) -> Result<(), Box<dyn std::error::Error>> {
        self.command_runner
            .run(std::process::Command::new("sh").arg("-c").arg(format!(
                "cd {} && if [ -r init.sh ]; then ./init.sh; fi",
                self.file_path_converter
                    .convert_to_os_path(directory_path)
                    .to_string_lossy()
            )))
    }
}
