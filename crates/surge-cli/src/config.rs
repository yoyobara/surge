use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(name = "surge", about = "a modern load testing tool", version, long_about = None)]
pub struct Config {
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Args, Debug, Clone)]
pub struct InitArgs {
    #[arg(value_name = "PATH", default_value = ".")]
    pub path: PathBuf,
}

#[derive(Args, Debug, Clone)]
pub struct RunArgs {}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Run(RunArgs),
    Init(InitArgs),
}
