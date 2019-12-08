use super::command_runner::CommandRunner;
use super::error::InfrastructureError;
use super::utilities;

#[derive(Default)]
pub struct ObjectLinker;

impl ObjectLinker {
    pub fn new() -> Self {
        Self
    }
}

impl app::ObjectLinker for ObjectLinker {
    fn link<'a>(
        &self,
        object_file_paths: impl IntoIterator<Item = &'a app::FilePath>,
        package_object_file_path: &app::FilePath,
    ) -> Result<(), Box<dyn std::error::Error>> {
        CommandRunner::run(
            std::process::Command::new(
                which::which("llvm-link")
                    .or_else(|_| which::which("llvm-link-10"))
                    .or_else(|_| which::which("llvm-link-9"))
                    .or_else(|_| which::which("llvm-link-8"))
                    .or_else(|_| Err(InfrastructureError::LlvmLinkNotFound))?,
            )
            .arg("-o")
            .arg(utilities::convert_to_os_path(package_object_file_path))
            .args(
                object_file_paths
                    .into_iter()
                    .map(utilities::convert_to_os_path),
            ),
        )?;

        Ok(())
    }
}
