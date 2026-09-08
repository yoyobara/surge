mod config;

use std::sync::Arc;

use clap::Parser;
use config::Config;

#[tokio::main]
async fn main() {
    let config = Arc::new(Config::parse());

    dbg!(config);
}
