use super::command_target::CommandTarget;

#[derive(Clone, Debug)]
pub enum Target {
    Command(CommandTarget),
    Library,
}
