use super::command_runner::CommandRunner;
use super::os_file_path_converter::OsFilePathConverter;

pub struct PreludePackageDownloader<'a> {
    command_runner: &'a CommandRunner,
    os_file_path_converter: &'a OsFilePathConverter,
    prelude_package_directory: std::path::PathBuf,
}

impl<'a> PreludePackageDownloader<'a> {
    pub fn new(
        command_runner: &'a CommandRunner,
        os_file_path_converter: &'a OsFilePathConverter,
        prelude_package_directory: impl AsRef<std::path::Path>,
    ) -> Self {
        Self {
            command_runner,
            os_file_path_converter,
            prelude_package_directory: prelude_package_directory.as_ref().into(),
        }
    }
}

impl<'a> app::PreludePackageDownloader for PreludePackageDownloader<'a> {
    fn download(&self, directory_path: &app::FilePath) -> Result<(), Box<dyn std::error::Error>> {
        let path = self
            .os_file_path_converter
            .convert_to_os_path(directory_path);

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
        )
    }
}
