pub fn convert_to_os_path(file_path: &app::FilePath) -> std::path::PathBuf {
    file_path
        .components()
        .iter()
        .collect::<std::path::PathBuf>()
}
