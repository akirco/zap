use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "A terminal file explorer", long_about = None)]
pub(crate) struct Cli {
    /// Initial directory to open
    #[arg(default_value = ".")]
    pub(crate) path: PathBuf,

    /// toml config path
    #[arg(short, long)]
    pub(crate) config: Option<PathBuf>,
}

pub(crate) fn parse_cli() -> Cli {
    Cli::parse()
}
