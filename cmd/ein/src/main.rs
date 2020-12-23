mod build;
mod builtin_configuration;
mod compile_configuration;
mod file_path_configuration;
mod init;
mod package_initialization_configuration;

use build::build;
use init::init;

fn main() {
    if let Err(error) = run() {
        infra::Logger::new().log_error(error.as_ref()).unwrap();
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    match clap::App::new("ein")
        .version("0.1.0")
        .setting(clap::AppSettings::SubcommandRequired)
        .subcommand(clap::SubCommand::with_name("build"))
        .subcommand(
            clap::SubCommand::with_name("init")
                .arg(
                    clap::Arg::with_name("target")
                        .possible_values(&["command", "library"])
                        .required(true),
                )
                .arg(clap::Arg::with_name("directory").required(true)),
        )
        .get_matches()
        .subcommand()
    {
        ("build", _) => build(),
        ("init", matches) => {
            let matches = matches.unwrap();

            init(
                matches.value_of("target").unwrap(),
                matches.value_of("directory").unwrap(),
            )
        }
        _ => unreachable!(),
    }
}
