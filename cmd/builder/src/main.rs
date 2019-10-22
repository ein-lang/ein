mod build;

use build::build;

fn main() {
    if let Err(error) = run() {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().count() > 1 {
        return Err(
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "too many arguments").into(),
        );
    }

    build(std::env::var("SLOTH_ROOT")?)?;

    Ok(())
}
