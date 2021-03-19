use super::application_target::ApplicationTarget;

#[derive(Clone, Debug, PartialEq)]
pub enum Target {
    Application(ApplicationTarget),
    Library,
}

impl From<ApplicationTarget> for Target {
    fn from(application_target: ApplicationTarget) -> Self {
        Target::Application(application_target)
    }
}
