use super::project_initialization_target::ProjectInitializationTarget;
use crate::build::{BuildConfiguration, CommandTarget, Target};
use crate::infra::{FilePath, FileStorage};

const DEFAULT_COMMAND_NAME: &str = "main";
const DEFAULT_MAIN_FILENAME: &str = "Main.ein";

pub struct ProjectInitializer<'a> {
    file_storage: &'a dyn FileStorage,
}

impl<'a> ProjectInitializer<'a> {
    pub fn new(file_storage: &'a dyn FileStorage) -> Self {
        Self { file_storage }
    }

    pub fn initialize(
        &self,
        target: ProjectInitializationTarget,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.file_storage.write(
            &FilePath::new(&["ein.json"]),
            serde_json::to_string_pretty(&BuildConfiguration::new(
                match target {
                    ProjectInitializationTarget::Command => {
                        Target::Command(CommandTarget::new(DEFAULT_COMMAND_NAME))
                    }
                    ProjectInitializationTarget::Library => Target::Library,
                },
                Default::default(),
            ))?
            .as_bytes(),
        )?;

        if target == ProjectInitializationTarget::Command {
            self.file_storage.write(
                &FilePath::new(&[DEFAULT_MAIN_FILENAME]),
                "main : System -> Number\nmain system = 0\n".as_bytes(),
            )?;
        }

        Ok(())
    }
}
