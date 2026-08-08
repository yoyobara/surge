use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(name = "surge-cli")]
#[command(about = "a modern load testing tool", long_about = None)]
pub struct Config {
    #[arg(value_name = "FILE")]
    pub file_path: PathBuf,

    #[arg(short, long, env = "SURGE_SOMETHING")]
    pub something: bool,
}
