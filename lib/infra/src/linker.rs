use super::command_runner::CommandRunner;
use super::utilities;

pub struct Linker {
    root_directory: Box<std::path::Path>,
}

impl Linker {
    pub fn new(root_directory: impl AsRef<std::path::Path>) -> Self {
        Self {
            root_directory: root_directory.as_ref().into(),
        }
    }
}

impl app::Linker for Linker {
    fn link(
        &self,
        object_file_paths: &[app::FilePath],
        command_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        CommandRunner::run(
            std::process::Command::new("clang")
                .arg("-o")
                .arg(command_name)
                .arg("-O3")
                .arg("-flto")
                .arg("-ldl")
                .arg("-lpthread")
                .args(
                    object_file_paths
                        .iter()
                        .map(|file_path| utilities::convert_to_os_path(file_path)),
                )
                .arg(self.root_directory.join("target/release/libruntime.a")),
        )
    }
}
