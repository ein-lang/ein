mod build;
mod environment;
mod error;

use build::build;

fn main() {
    if std::env::args().count() > 1 {
        handle_error::<(), std::io::Error>(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "too many arguments",
        ));
    }

    build(environment::root_directory().unwrap_or_else(handle_error)).unwrap_or_else(handle_error);
}

fn handle_error<T, E: std::error::Error + std::fmt::Display>(error: E) -> T {
    eprintln!("{}", error);
    std::process::exit(1);
}
