mod cli;
mod core;

use crate::cli::args::CLIArgs;
use crate::core::copy::{copy, multiple_copy};
use crate::core::progress_bar::ProgressBarStyle;
use clap::Parser;

#[tokio::main]
async fn main() {
    let args = CLIArgs::parse();
    let style = match args.style.as_deref() {
        Some("minimal") => ProgressBarStyle::Minimal,
        Some("detailed") => ProgressBarStyle::Detailed,
        _ => ProgressBarStyle::Default,
    };
    let result = if args.sources.len() == 1 {
        copy(&args.sources[0], &args.destination, style).await
    } else {
        multiple_copy(args.sources, args.destination, style).await
    };
    match result {
        Ok(_) => println!("File copied successfully."),
        Err(e) => eprintln!("Error copying file: {}", e),
    }
}
