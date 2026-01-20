use clap::ValueEnum;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum ProgressBarStyle {
    #[default]
    Default,
    Detailed,
}

impl ProgressBarStyle {
    pub fn apply(&self, pb: &ProgressBar, total_files: usize) {
        let style = match self {
            ProgressBarStyle::Default => ProgressStyle::default_bar()
                .template("{msg} {percent}% {wide_bar} ETA:{eta_precise}")
                .unwrap()
                .progress_chars("▓░░"),

            ProgressBarStyle::Detailed => ProgressStyle::default_bar()
                .template(
                    "{msg} {wide_bar} {percent:>3}% • {binary_bytes}/{binary_total_bytes} • {binary_bytes_per_sec} • Elapsed: {elapsed_precise} • ETA: {eta_precise}"
                )
                .unwrap()
                .progress_chars("▓░░"),
        };

        pb.set_style(style);
        if matches!(self, ProgressBarStyle::Detailed) {
            pb.set_message(format!("Copying: 0/{} files", total_files));
        } else {
            pb.set_message("Copying");
        }
    }
}
