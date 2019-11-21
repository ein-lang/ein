use super::command_runner::CommandRunner;
use super::file_storage::FileStorage;

pub struct Archiver<'a> {
    object_file_storage: &'a FileStorage,
    interface_file_storage: &'a FileStorage,
}

impl<'a> Archiver<'a> {
    pub fn new(
        object_file_storage: &'a FileStorage,
        interface_file_storage: &'a FileStorage,
    ) -> Self {
        Self {
            object_file_storage,
            interface_file_storage,
        }
    }
}

impl<'a> app::Archiver for Archiver<'a> {
    fn archive(
        &self,
        object_file_paths: &[app::FilePath],
        interface_file_paths: &[app::FilePath],
    ) -> Result<(), std::io::Error> {
        CommandRunner::run(
            std::process::Command::new("clang")
                .arg("-c")
                .arg("-o")
                .arg("library.o")
                .args(
                    object_file_paths
                        .iter()
                        .map(|file_path| self.object_file_storage.resolve_file_path(file_path)),
                ),
        )?;

        CommandRunner::run(
            std::process::Command::new("ar")
                .arg("-r")
                .arg("library.a")
                .arg("library.o"),
        )?;

        let mut builder = tar::Builder::new(std::fs::File::create("library.tar")?);
        builder.append_path("library.a")?;

        for file_path in interface_file_paths {
            let path = self.interface_file_storage.resolve_file_path(file_path);

            builder.append_file(&path, &mut std::fs::File::open(&path)?)?;
        }

        Ok(())
    }
}
