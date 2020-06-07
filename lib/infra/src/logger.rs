use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Default)]
pub struct Logger {}

impl Logger {
    pub fn new() -> Self {
        Self {}
    }

    pub fn log_error(
        &self,
        error: &dyn std::error::Error,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut stderr = StandardStream::stderr(ColorChoice::Auto);

        stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
        write!(&mut stderr, "error")?;
        stderr.set_color(ColorSpec::new().set_fg(None))?;

        writeln!(
            &mut stderr,
            ": {}",
            format!("{}", error).replace("\n", "\n  ").trim()
        )?;

        if let Some(error) = error.source() {
            self.log_error(error)?;
        }

        Ok(())
    }
}

impl app::Logger for Logger {
    fn log(&self, log: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut stderr = StandardStream::stderr(ColorChoice::Auto);

        stderr.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        write!(&mut stderr, "info")?;
        stderr.set_color(ColorSpec::new().set_fg(None))?;

        writeln!(&mut stderr, ": {}", log)?;

        Ok(())
    }
}
