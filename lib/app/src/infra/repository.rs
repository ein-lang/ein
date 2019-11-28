pub trait Repository {
    fn get_package(&self) -> Result<ein::Package, Box<dyn std::error::Error>>;
}
