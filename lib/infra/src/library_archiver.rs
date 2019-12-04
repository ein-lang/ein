use super::utilities;
use std::collections::HashMap;

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
        interface_file_paths: &HashMap<app::FilePath, app::FilePath>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut builder = tar::Builder::new(std::fs::File::create("library.tar")?);

        builder.append_path_with_name(
            &utilities::convert_to_os_path(object_file_path),
            &utilities::convert_to_os_path(archive_object_file_path),
        )?;

        for (relative_interface_file_path, interface_file_path) in interface_file_paths {
            builder.append_file(
                &utilities::convert_to_os_path(relative_interface_file_path),
                &mut std::fs::File::open(&utilities::convert_to_os_path(interface_file_path))?,
            )?;
        }

        Ok(())
    }
}
