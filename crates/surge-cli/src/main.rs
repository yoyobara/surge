mod config;
mod init;
mod run;
mod show;

use std::sync::Arc;

use clap::Parser;
use config::{Commands, Config};
use init::handle_init;

use crate::{run::handle_run, show::handle_show};

fn main() {
    let config = Arc::new(Config::parse());

    match &config.command {
        Commands::Init(args) => {
            if let Err(err) = handle_init(args) {
                eprintln!("Error initializing project: {err}");
                std::process::exit(1);
            }
        }
        Commands::Run(args) => {
            if let Err(err) = handle_run(args) {
                eprintln!("Error running project: {err}");
                std::process::exit(1);
            }
        }
        Commands::Show(args) => {
            if let Err(err) = handle_show(args) {
                eprintln!("error listing tests: {err}");
                std::process::exit(1);
            }
        }
    }
}
