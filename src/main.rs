use agent_policy::cli::{Cli, Command};
use clap::Parser;
use std::process;

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Init { force } => agent_policy::commands::init::run(force),
        Command::Generate { config, targets } => {
            agent_policy::commands::generate::run(&config, targets.as_deref())
        }
        Command::Check { config, targets } => {
            agent_policy::commands::check::run(&config, targets.as_deref())
        }
        Command::Lint { config } => agent_policy::commands::lint::run(&config),
        Command::ListTargets => {
            agent_policy::commands::list_targets::run();
            Ok(())
        }
        Command::InstallHooks { pre_push } => agent_policy::commands::install_hooks::run(pre_push),
        Command::Import => agent_policy::commands::import::run(),
    };

    if let Err(e) = result {
        eprintln!("error: {e}");
        process::exit(1);
    }
}
