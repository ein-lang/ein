use crate::common::{BuildConfiguration, Target};
use crate::common::{FilePath, StaticFilePathManager};
use crate::infra::FileSystem;

pub struct ProjectInitializer<'a> {
    file_system: &'a dyn FileSystem,
    static_file_path_manager: &'a StaticFilePathManager,
}

impl<'a> ProjectInitializer<'a> {
    pub fn new(
        file_system: &'a dyn FileSystem,
        static_file_path_manager: &'a StaticFilePathManager,
    ) -> Self {
        Self {
            file_system,
            static_file_path_manager,
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
                self.static_file_path_manager.main_source_file_path(),
                "main : System -> Number\nmain system = 0\n".as_bytes(),
            )?;
        }

        Ok(())
    }
}
