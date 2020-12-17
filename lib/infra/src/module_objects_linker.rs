use super::command_runner::CommandRunner;
use super::error::InfrastructureError;
use super::file_path_converter::FilePathConverter;

pub struct ModuleObjectsLinker<'a> {
    command_runner: &'a CommandRunner,
    file_path_converter: &'a FilePathConverter,
}

impl<'a> ModuleObjectsLinker<'a> {
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

impl<'a> app::ModuleObjectsLinker for ModuleObjectsLinker<'a> {
    fn link(
        &self,
        object_file_paths: &[app::FilePath],
        package_object_file_path: &app::FilePath,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.command_runner.run(
            std::process::Command::new(
                which::which("llvm-link")
                    .or_else(|_| which::which("llvm-link-10"))
                    .or_else(|_| which::which("llvm-link-9"))
                    .or_else(|_| which::which("llvm-link-8"))
                    .map_err(|_| InfrastructureError::LlvmLinkNotFound)?,
            )
            .arg("-o")
            .arg(
                self.file_path_converter
                    .convert_to_os_path(package_object_file_path),
            )
            .args(
                object_file_paths
                    .iter()
                    .map(|path| self.file_path_converter.convert_to_os_path(path)),
            ),
        )?;

        Ok(())
    }
}
