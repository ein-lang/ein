use crate::build::{BuildConfiguration, FilePathConfiguration, Target};
use crate::infra::{FilePath, FileStorage};

pub struct ProjectInitializer<'a> {
    file_storage: &'a dyn FileStorage,
    file_path_configuration: &'a FilePathConfiguration,
}

impl<'a> ProjectInitializer<'a> {
    pub fn new(
        file_storage: &'a dyn FileStorage,
        file_path_configuration: &'a FilePathConfiguration,
    ) -> Self {
        Self {
            file_storage,
            file_path_configuration,
        }
    }

    pub fn initialize(&self, target: &Target) -> Result<(), Box<dyn std::error::Error>> {
        self.file_storage.write(
            &FilePath::new(&["ein.json"]),
            serde_json::to_string_pretty(&BuildConfiguration::new(
                target.clone(),
                Default::default(),
            ))?
            .as_bytes(),
        )?;

        if matches!(target, Target::Command(_)) {
            self.file_storage.write(
                self.file_path_configuration.main_source_file_path(),
                "main : System -> Number\nmain system = 0\n".as_bytes(),
            )?;
        }

        Ok(())
    }
}
