use super::error::InfrastructureError;
use std::io::Write;

pub struct CommandRunner;

impl CommandRunner {
    pub fn run(command: &mut std::process::Command) -> Result<(), Box<dyn std::error::Error>> {
        let output = command.output()?;

        if output.status.success() {
            return Ok(());
        }

        std::io::stderr().write_all(&output.stdout)?;
        std::io::stderr().write_all(&output.stderr)?;

        Err(InfrastructureError::CommandExit {
            status_code: output.status.code(),
        }
        .into())
    }
}
