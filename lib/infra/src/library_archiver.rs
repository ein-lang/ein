use super::utilities;

#[derive(Default)]
pub struct LibraryArchiver;

impl LibraryArchiver {
    pub fn new() -> Self {
        Self
    }
}

impl app::LibraryArchiver for LibraryArchiver {
    fn archive(
        &self,
        object_file_path: &app::FilePath,
        archive_object_file_path: &app::FilePath,
        interface_file_path: &app::FilePath,
        archive_interface_file_path: &app::FilePath,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut builder = tar::Builder::new(std::fs::File::create("library.tar")?);

        builder.append_path_with_name(
            &utilities::convert_to_os_path(object_file_path),
            &utilities::convert_to_os_path(archive_object_file_path),
        )?;

        builder.append_path_with_name(
            &utilities::convert_to_os_path(interface_file_path),
            &utilities::convert_to_os_path(archive_interface_file_path),
        )?;

        Ok(())
    }
}
