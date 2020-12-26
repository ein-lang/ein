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
    fn initialize(
        &self,
        directory_path: &app::FilePath,
    ) -> Result<Option<app::FilePath>, Box<dyn std::error::Error>> {
        let stdout = self
            .command_runner
            .run(std::process::Command::new("sh").arg("-c").arg(format!(
                "cd {} && if [ -r ffi.sh ]; then ./ffi.sh; fi",
                self.file_path_converter
                    .convert_to_os_path(directory_path)
                    .to_string_lossy()
            )))?;

        Ok(if stdout == "" {
            None
        } else {
            Some(directory_path.join(&self.file_path_converter.convert_to_file_path(stdout)?))
        })
    }
}
