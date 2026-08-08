use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "surge")]
#[command(about = "a modern load testing tool", long_about = None)]
pub struct Config {
    #[arg(value_name = "TEST_NAME")]
    pub test_name: String,
}
