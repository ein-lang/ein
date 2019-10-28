pub struct InitializerConfiguration {
    name: String,
    dependent_initializer_names: Vec<String>,
}

impl InitializerConfiguration {
    pub fn new(name: String, dependent_initializer_names: Vec<String>) -> Self {
        Self {
            name,
            dependent_initializer_names,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn dependent_initializer_names(&self) -> &[String] {
        &self.dependent_initializer_names
    }
}
