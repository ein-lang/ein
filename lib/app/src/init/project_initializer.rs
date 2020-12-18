use crate::common::{
    BuildConfiguration, FilePath, FilePathConfiguration, StaticFilePathManager, Target,
};
use crate::infra::FileSystem;

pub struct ProjectInitializer<'a> {
    file_system: &'a dyn FileSystem,
    static_file_path_manager: &'a StaticFilePathManager,
    file_path_configuration: &'a FilePathConfiguration,
}

impl<'a> ProjectInitializer<'a> {
    pub fn new(
        file_system: &'a dyn FileSystem,
        static_file_path_manager: &'a StaticFilePathManager,
        file_path_configuration: &'a FilePathConfiguration,
    ) -> Self {
        Self {
            file_system,
            static_file_path_manager,
            file_path_configuration,
        }
    }

    pub fn initialize(&self, target: &Target) -> Result<(), Box<dyn std::error::Error>> {
        self.file_system.write(
            &FilePath::new(&[self.file_path_configuration.build_configuration_filename]),
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
