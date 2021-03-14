use super::command_runner::CommandRunner;
use super::file_path_converter::FilePathConverter;

pub struct ApplicationLinker<'a> {
    command_runner: &'a CommandRunner,
    file_path_converter: &'a FilePathConverter,
}

impl<'a> ApplicationLinker<'a> {
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

impl<'a> app::ApplicationLinker for ApplicationLinker<'a> {
    fn link(
        &self,
        object_file_paths: &[app::FilePath],
        application_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.command_runner.run(
            std::process::Command::new("clang")
                .arg("-Werror") // cspell:disable-line
                .arg("-Wno-incompatible-pointer-types-discards-qualifiers") // cspell:disable-line
                .arg("-o")
                .arg(
                    self.file_path_converter
                        .convert_to_os_path(&app::FilePath::new(&[application_name])),
                )
                .arg("-O3")
                .arg("-flto")
                .args(
                    object_file_paths
                        .iter()
                        .map(|path| self.file_path_converter.convert_to_os_path(path)),
                )
                .arg("-ldl")
                .arg("-lpthread"),
        )?;

        Ok(())
    }
}
