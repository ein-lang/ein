use super::command_runner::CommandRunner;
use super::file_path_converter::FilePathConverter;

pub struct CommandLinker<'a> {
    command_runner: &'a CommandRunner,
    file_path_converter: &'a FilePathConverter,
    runtime_library_path: std::path::PathBuf,
}

impl<'a> CommandLinker<'a> {
    pub fn new(
        command_runner: &'a CommandRunner,
        file_path_converter: &'a FilePathConverter,
        runtime_library_path: impl AsRef<std::path::Path>,
    ) -> Self {
        Self {
            command_runner,
            file_path_converter,
            runtime_library_path: runtime_library_path.as_ref().into(),
        }
    }
}

impl<'a> app::CommandLinker for CommandLinker<'a> {
    fn link(
        &self,
        object_file_paths: &[app::FilePath],
        command_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.command_runner.run(
            std::process::Command::new("clang")
                .arg("-Werror") // cspell:disable-line
                .arg("-Wno-incompatible-pointer-types-discards-qualifiers") // cspell:disable-line
                .arg("-o")
                .arg(self.file_path_converter.convert_to_os_path(&app::FilePath::new(&[command_name])))
                .arg("-O3")
                // TODO Replace this with the one of system packages.
                // This double linking of the runtime library is to define
                // the global allocator symbols in Rust first.
                .arg(&self.runtime_library_path)
                .args(
                    object_file_paths
                        .iter()
                        .map(|path| self.file_path_converter.convert_to_os_path(path)),
                )
                .arg(&self.runtime_library_path)
                .arg("-ldl")
                .arg("-lpthread"),
        )?;

        Ok(())
    }
}
