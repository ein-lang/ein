use lazy_static::lazy_static;

pub const PACKAGE_CONFIGURATION_FILENAME: &str = "ein.json";

lazy_static! {
    pub static ref FILE_PATH_CONFIGURATION: app::FilePathConfiguration =
        app::FilePathConfiguration::new(
            PACKAGE_CONFIGURATION_FILENAME,
            ".ein",
            "package",
            "ein",
            "bc",
            "json",
            "Main"
        );
}
