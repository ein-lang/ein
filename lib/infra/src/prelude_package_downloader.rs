use super::command_runner::CommandRunner;
use super::file_path_converter::FilePathConverter;

pub struct PreludePackageDownloader<'a> {
    command_runner: &'a CommandRunner,
    file_path_converter: &'a FilePathConverter,
    prelude_package_directory: std::path::PathBuf,
}

impl<'a> PreludePackageDownloader<'a> {
    pub fn new(
        command_runner: &'a CommandRunner,
        file_path_converter: &'a FilePathConverter,
        prelude_package_directory: impl AsRef<std::path::Path>,
    ) -> Self {
        Self {
            command_runner,
            file_path_converter,
            prelude_package_directory: prelude_package_directory.as_ref().into(),
        }
    }
}

impl<'a> app::PreludePackageDownloader for PreludePackageDownloader<'a> {
    fn download(&self, directory_path: &app::FilePath) -> Result<(), Box<dyn std::error::Error>> {
        let path = self.file_path_converter.convert_to_os_path(directory_path);

        if path.exists() {
            return Ok(());
        } else if let Some(path) = path.parent() {
            std::fs::create_dir_all(&path)?;
        }

        self.command_runner.run(
            std::process::Command::new("cp")
                .arg("-r")
                .arg(&self.prelude_package_directory)
                .arg(path),
        )?;

        Ok(())
    }
}
