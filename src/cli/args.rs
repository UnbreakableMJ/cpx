use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct CLIArgs {
    #[arg(required = true)]
    pub sources: Vec<PathBuf>,
    #[arg(required = true)]
    pub destination: PathBuf,

    #[arg(short, long, help = "Progress bar style: default, minimal, detailed")]
    pub style: Option<String>,
}
