use agent_policy::cli::{Cli, Command};
use clap::Parser;
use std::process;

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Init { force } => agent_policy::commands::init::run(force),
        Command::Generate { config } => agent_policy::commands::generate::run(&config),
        Command::Check { config } => agent_policy::commands::check::run(&config),
        Command::ListTargets => {
            agent_policy::commands::list_targets::run();
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("error: {e}");
        process::exit(1);
    }
}
