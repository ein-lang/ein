use crate::error::BuildError;
use std::io::Write;
use std::path::Path;

const BC_PATH: &str = "main.bc";

pub fn build(root_directory: String) -> Result<(), BuildError> {
    run_command(
        std::process::Command::new(Path::new(&root_directory).join("target/release/compiler"))
            .arg("main.sl")
            .arg(BC_PATH),
    )?;

    run_command(
        std::process::Command::new("clang")
            .arg("-O3")
            .arg("-flto")
            .arg("-ldl")
            .arg("-lpthread")
            .arg(BC_PATH)
            .arg(Path::new(&root_directory).join("target/release/libruntime.a")),
    )?;

    Ok(())
}

fn run_command(command: &mut std::process::Command) -> Result<(), BuildError> {
    let output = command.output()?;

    if output.status.success() {
        Ok(())
    } else {
        std::io::stderr().write_all(&output.stdout)?;
        std::io::stderr().write_all(&output.stderr)?;

        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            output
                .status
                .code()
                .map(|code| format!("a command exited with status code {}", code))
                .unwrap_or_else(|| "a command exited with no status code".into()),
        )
        .into())
    }
}
