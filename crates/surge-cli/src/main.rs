mod config;
mod init;
mod show;

use std::sync::Arc;

use clap::Parser;
use config::{Commands, Config};
use init::handle_init;

use crate::show::handle_show;

#[tokio::main]
async fn main() {
    let config = Arc::new(Config::parse());

    match &config.command {
        Commands::Init(args) => {
            if let Err(err) = handle_init(args) {
                eprintln!("Error initializing project: {err}");
                std::process::exit(1);
            }
        }
        Commands::Run(args) => {
            dbg!(args);
        }
        Commands::Show(args) => {
            if let Err(err) = handle_show(args) {
                eprintln!("error listing tests: {err}");
            }
        }
    }
}
