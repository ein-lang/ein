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
            std::process::Command::new("llvm-link")
                .arg("-o")
                .arg("library.bc")
                .args(object_file_paths.iter().map(|file_path| {
                    self.object_file_storage
                        .resolve_absolute_file_path(file_path)
                })),
        )?;

        let mut builder = tar::Builder::new(std::fs::File::create("library.tar")?);
        builder.append_path("library.bc")?;
        std::fs::remove_file("library.bc")?;

        for file_path in interface_file_paths {
            builder.append_file(
                &self
                    .interface_file_storage
                    .resolve_relative_file_path(file_path),
                &mut std::fs::File::open(
                    &self
                        .interface_file_storage
                        .resolve_absolute_file_path(file_path),
                )?,
            )?;
        }

        Ok(())
    }
}
