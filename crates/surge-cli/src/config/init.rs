use clap::{Args, ValueEnum};
use std::path::PathBuf;

/// Options for the `init` subcommand
#[derive(Args, Debug, Clone)]
pub struct InitArgs {
    /// Target directory or scenario file name
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Template type to initialize
    #[arg(short, long, value_enum, default_value_t = TemplateType::Http)]
    pub template: TemplateType,

    /// Overwrite destination if it already exists
    #[arg(short, long)]
    pub force: bool,
}

/// Supported project templates for the `init` subcommand
#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateType {
    Http,
    Websocket,
    Grpc,
}
