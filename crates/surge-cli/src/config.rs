pub mod init;
pub mod run;

use clap::{Parser, Subcommand};

pub use init::InitArgs;
pub use run::RunArgs;

#[derive(Parser, Debug, Clone)]
#[command(name = "surge", about = "a modern load testing tool", version, long_about = None)]
pub struct Config {
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Run(RunArgs),
    Init(InitArgs),
}
