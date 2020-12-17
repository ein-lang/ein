use crate::common::{BuildConfiguration, Target};
use crate::common::{FilePath, FilePathConfiguration};
use crate::infra::FileSystem;

pub struct ProjectInitializer<'a> {
    file_system: &'a dyn FileSystem,
    file_path_configuration: &'a FilePathConfiguration,
}

impl<'a> ProjectInitializer<'a> {
    pub fn new(
        file_system: &'a dyn FileSystem,
        file_path_configuration: &'a FilePathConfiguration,
    ) -> Self {
        Self {
            file_system,
            file_path_configuration,
        }
    }

    pub fn initialize(&self, target: &Target) -> Result<(), Box<dyn std::error::Error>> {
        self.file_system.write(
            &FilePath::new(&["ein.json"]),
            serde_json::to_string_pretty(&BuildConfiguration::new(
                target.clone(),
                Default::default(),
            ))?
            .as_bytes(),
        )?;

        if matches!(target, Target::Command(_)) {
            self.file_system.write(
                self.file_path_configuration.main_source_file_path(),
                "main : System -> Number\nmain system = 0\n".as_bytes(),
            )?;
        }

        Ok(())
    }
}
