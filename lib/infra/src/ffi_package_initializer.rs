use super::command_runner::CommandRunner;
use super::file_path_converter::FilePathConverter;

const FFI_INITIALIZATION_SCRIPT: &str = "ein-ffi.sh";

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
                "cd {} && if [ -r {script} ]; then ./{script}; fi",
                self.file_path_converter
                    .convert_to_os_path(directory_path)
                    .to_string_lossy(),
                script = FFI_INITIALIZATION_SCRIPT,
            )))?;
        let path_string = stdout.trim();

        Ok(if path_string.is_empty() {
            None
        } else {
            Some(
                directory_path.join(
                    &self
                        .file_path_converter
                        .convert_relative_to_file_path(path_string)?,
                ),
            )
        })
    }

    fn is_ffi_used(&self, directory_path: &app::FilePath) -> bool {
        self.file_path_converter
            .convert_to_os_path(
                &directory_path.join(&app::FilePath::new(vec![FFI_INITIALIZATION_SCRIPT])),
            )
            .exists()
    }
}
