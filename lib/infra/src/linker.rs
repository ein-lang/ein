use super::file_storage::FileStorage;
use std::io::Write;

pub struct Linker<'a> {
    package: &'a sloth::Package,
    root_directory: Box<std::path::Path>,
    object_file_storage: &'a FileStorage,
}

impl<'a> Linker<'a> {
    pub fn new(
        package: &'a sloth::Package,
        root_directory: impl AsRef<std::path::Path>,
        object_file_storage: &'a FileStorage,
    ) -> Self {
        Self {
            package,
            root_directory: root_directory.as_ref().into(),
            object_file_storage,
        }
    }
}

impl<'a> app::Linker for Linker<'a> {
    fn link(&self, file_path: &app::FilePath) -> Result<(), std::io::Error> {
        let output = std::process::Command::new("clang")
            .arg("-o")
            .arg(self.package.name())
            .arg("-O3")
            .arg("-flto")
            .arg("-ldl")
            .arg("-lpthread")
            .arg(self.object_file_storage.resolve_file_path(file_path))
            .arg(self.root_directory.join("target/release/libruntime.a"))
            .output()?;

        if output.status.success() {
            return Ok(());
        }

        std::io::stderr().write_all(&output.stdout)?;
        std::io::stderr().write_all(&output.stderr)?;

        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            output
                .status
                .code()
                .map(|code| format!("a command exited with status code {}", code))
                .unwrap_or_else(|| "a command exited with no status code".into()),
        ))
    }
}
