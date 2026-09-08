use clap::{Args, ValueEnum};

/// Options for the `run` subcommand
#[derive(Args, Debug, Clone)]
pub struct RunArgs {
    /// Target URL or path to a test script (e.g., tests/test1.lua)
    #[arg(value_name = "TARGET")]
    pub target: String,

    /// Number of concurrent virtual users / connections
    #[arg(short = 'c', long, default_value_t = 10)]
    pub concurrency: usize,

    /// Total duration of the test (e.g., "30s", "2m")
    #[arg(short, long, default_value = "30s")]
    pub duration: String,

    /// Request rate limit (requests per second; 0 = unlimited)
    #[arg(short, long, default_value_t = 0)]
    pub rate: u64,

    /// HTTP request method to use
    #[arg(short = 'm', long, value_enum, default_value_t = HttpMethod::Get)]
    pub method: HttpMethod,

    /// Custom headers to include (e.g. -H "Authorization: Bearer token")
    #[arg(short = 'H', long = "header")]
    pub headers: Vec<String>,

    /// Timeout in seconds for each request
    #[arg(short = 't', long, default_value_t = 30, env = "SURGE_TIMEOUT")]
    pub timeout: u64,
}

/// Supported HTTP methods for the `run` subcommand
#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
}
