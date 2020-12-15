mod build;
mod compile_configuration;
mod file_path_configuration;
mod init;

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
                .arg(clap::Arg::with_name("target").index(1).required(true)),
        )
        .get_matches()
        .subcommand()
    {
        ("build", _) => build(),
        ("init", matches) => init(matches.unwrap().value_of("target").unwrap()),
        _ => unreachable!(),
    }
}
