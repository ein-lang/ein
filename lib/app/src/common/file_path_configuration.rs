pub const EXTERNAL_PACKAGES_DIRECTORY: &str = "packages";
pub const INTERFACE_FILE_EXTENSION: &str = "json";
pub const OBJECT_DIRECTORY: &str = "objects";
pub const OBJECT_FILE_EXTENSION: &str = "bc";
pub const PRELUDE_PACKAGE_DIRECTORY: &str = "prelude";

pub struct FilePathConfiguration {
    pub build_configuration_filename: &'static str,
    pub output_directory_name: &'static str,
    pub source_file_extension: &'static str,
    pub main_file_basename: &'static str,
}
