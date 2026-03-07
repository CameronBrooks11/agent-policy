use agent_policy::cli::{Cli, Command};
use clap::Parser;
use std::process;

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Init { force } => agent_policy::commands::init::run(force),
        Command::Generate { config } => agent_policy::commands::generate::run(&config),
        // `check` is implemented in Phase 3.
        Command::Check { config: _ } => {
            eprintln!("error: `check` is not yet implemented (Phase 3)");
            process::exit(1);
        }
    };

    if let Err(e) = result {
        eprintln!("error: {e}");
        process::exit(1);
    }
}
