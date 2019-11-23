pub trait ExternalPackageInitializer {
    fn initialize(&self) -> Result<(), Box<dyn std::error::Error>>;
}
