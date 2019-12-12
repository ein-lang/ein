pub fn convert_to_os_path(file_path: &app::FilePath) -> std::path::PathBuf {
    file_path.components().collect::<std::path::PathBuf>()
}

pub fn convert_to_file_path(path: impl AsRef<std::path::Path>) -> app::FilePath {
    app::FilePath::new(
        path.as_ref()
            .components()
            .filter_map(|component| match component {
                std::path::Component::Normal(component) => Some(component.to_string_lossy().into()),
                _ => None,
            })
            .collect::<Vec<String>>(),
    )
}
