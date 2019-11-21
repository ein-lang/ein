use std::io::Write;

pub struct CommandRunner;

impl CommandRunner {
    pub fn run(command: &mut std::process::Command) -> Result<(), std::io::Error> {
        let output = command.output()?;

        if output.status.success() {
            return Ok(());
        }

        std::io::stderr().write_all(&output.stdout)?;
        std::io::stderr().write_all(&output.stderr)?;

        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            output
                .status
                .code()
                .map(|code| format!("a command exited with status code {}", code))
                .unwrap_or_else(|| "a command exited with no status code".into()),
        ))
    }
}
