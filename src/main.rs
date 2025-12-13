use clap::Parser;
use cpx::cli::args::CLIArgs;
use cpx::core::copy::{copy, multiple_copy};
use cpx::utility::progress_bar::ProgressBarStyle;
use std::process;

#[tokio::main]
async fn main() {
    let args = CLIArgs::parse();
    let style = match args.style.as_deref() {
        Some("minimal") => ProgressBarStyle::Minimal,
        Some("detailed") => ProgressBarStyle::Detailed,
        _ => ProgressBarStyle::Default,
    };
    let (sources, destination, options) = match args.validate() {
        Ok(validated) => validated,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };
    let result = if sources.len() == 1 {
        copy(&sources[0], &destination, style, &options).await
    } else {
        multiple_copy(sources, destination, style, &options).await
    };
    
    if let Err(e) = result {
        eprintln!("Error copying file: {}", e);  
        process::exit(1);  
    }
}
