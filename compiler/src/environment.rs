pub fn root_directory() -> Result<String, std::env::VarError> {
    std::env::var("SLOTH_ROOT")
}
