use indicatif::{ProgressBar, ProgressStyle};

#[derive(Debug, Clone, Copy)]
pub enum ProgressBarStyle {
    Default,
    Minimal,
}

impl ProgressBarStyle {
    pub fn apply(&self, pb: &ProgressBar) {
        let style = match self {
            ProgressBarStyle::Minimal => ProgressStyle::default_bar()
                .template("{spinner} {msg:20} [{bar:65}] {percent:>3}%")
                .unwrap()
                .progress_chars("━╾─"),
            ProgressBarStyle::Default => ProgressStyle::default_bar()
                .template("{spinner} {msg:20} [{bar:65}] {binary_bytes:>5}/{binary_total_bytes:<5} • {binary_bytes_per_sec:>5}")
                .unwrap()
                .progress_chars("━╾─"),
        };
        pb.set_style(style);
    }
}

impl Default for ProgressBarStyle {
    fn default() -> Self {
        ProgressBarStyle::Default
    }
}

pub fn apply_overall(pb: &ProgressBar) {
    let style = ProgressStyle::default_spinner()
        .template(
            "{msg} \
             • {binary_bytes:>5}/{binary_total_bytes:<5} \
             • ETA {eta_precise} \n",
        )
        .unwrap();

    pb.set_style(style);
}
