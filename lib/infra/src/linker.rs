use super::command_runner::CommandRunner;
use super::file_storage::FileStorage;

pub struct Linker<'a> {
    root_directory: Box<std::path::Path>,
    object_file_storage: &'a FileStorage,
}

impl<'a> Linker<'a> {
    pub fn new(
        root_directory: impl AsRef<std::path::Path>,
        object_file_storage: &'a FileStorage,
    ) -> Self {
        Self {
            root_directory: root_directory.as_ref().into(),
            object_file_storage,
        }
    }
}

impl<'a> app::Linker for Linker<'a> {
    fn link(&self, file_paths: &[app::FilePath], command_name: &str) -> Result<(), std::io::Error> {
        CommandRunner::run(
            std::process::Command::new("clang")
                .arg("-o")
                .arg(command_name)
                .arg("-O3")
                .arg("-flto")
                .arg("-ldl")
                .arg("-lpthread")
                .args(file_paths.iter().map(|file_path| {
                    self.object_file_storage
                        .resolve_absolute_file_path(file_path)
                }))
                .arg(self.root_directory.join("target/release/libruntime.a")),
        )
    }
}
