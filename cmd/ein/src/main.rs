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
    match clap::Command::new("ein")
        .version("0.1.0")
        .subcommand_required(true)
        .subcommand(clap::Command::new("build").about("Builds a package"))
        .subcommand(
            clap::Command::new("init")
                .arg(
                    clap::Arg::new("lib")
                        .short('l')
                        .long("lib")
                        .help("Creates a library package"),
                )
                .arg(
                    clap::Arg::new("directory")
                        .required(true)
                        .help("Specifies a package directory"),
                )
                .about("Initializes a package"),
        )
        .get_matches()
        .subcommand()
        .unwrap()
    {
        ("build", _) => build(),
        ("init", matches) => init(
            matches.value_of("directory").unwrap(),
            matches.is_present("lib"),
        ),
        _ => unreachable!(),
    }
}
