use super::application_target::ApplicationTarget;

#[derive(Clone, Debug, PartialEq)]
pub enum Target {
    Application(ApplicationTarget),
    Library,
}

impl Target {
    pub fn as_application(&self) -> Option<&ApplicationTarget> {
        match self {
            Self::Application(application) => Some(application),
            Self::Library => None,
        }
    }
}

impl From<ApplicationTarget> for Target {
    fn from(application_target: ApplicationTarget) -> Self {
        Target::Application(application_target)
    }
}
