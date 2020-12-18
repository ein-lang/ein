use lazy_static::lazy_static;

pub const PACKAGE_CONFIGURATION_FILENAME: &str = "ein.json";

lazy_static! {
    pub static ref FILE_PATH_CONFIGURATION: app::FilePathConfiguration =
        app::FilePathConfiguration {
            build_configuration_filename: PACKAGE_CONFIGURATION_FILENAME,
            output_directory_name: ".ein",
            package_artifact_basename: "package",
            source_file_extension: "ein",
            object_file_extension: "bc",
            interface_file_extension: "json",
            main_file_basename: "Main"
        };
}
