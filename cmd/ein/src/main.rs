mod build;
mod compile_configuration;
mod file_path_configuration;
mod init;
mod package_initialization_configuration;
mod system_package_configuration;

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
        .subcommand(clap::SubCommand::with_name("build").about("Builds a package"))
        .subcommand(
            clap::SubCommand::with_name("init")
                .arg(
                    clap::Arg::with_name("lib")
                        .short("l")
                        .long("lib")
                        .help("Creates a library package"),
                )
                .arg(
                    clap::Arg::with_name("directory")
                        .required(true)
                        .help("Specifies a package directory"),
                )
                .about("Initializes a package"),
        )
        .get_matches()
        .subcommand()
    {
        ("build", _) => build(),
        ("init", matches) => {
            let matches = matches.unwrap();

            init(
                matches.value_of("directory").unwrap(),
                matches.is_present("lib"),
            )
        }
        _ => unreachable!(),
    }
}
