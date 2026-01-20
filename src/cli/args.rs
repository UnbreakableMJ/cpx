use crate::utility::{preserve::PreserveAttr, progress_bar::ProgressBarStyle};
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SymlinkMode {
    Auto,
    Absolute,
    Relative,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq)]
pub enum ReflinkMode {
    Always,
    Auto,
    Never,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq)]
pub enum BackupMode {
    None,
    Numbered,
    Existing,
    Simple,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum FollowSymlink {
    NoDereference,
    Dereference,
    CommandLineSymlink,
}

#[derive(Parser, Debug)]
pub struct CLIArgs {
    #[arg(required = true)]
    pub sources: Vec<PathBuf>,

    #[arg(required = true)]
    pub destination: PathBuf,

    #[arg(
        short = 't',
        long = "target-directory",
        value_name = "DIRECTORY",
        help = "copy all SOURCE arguments into DIRECTORY"
    )]
    pub target_directory: Option<PathBuf>,

    #[arg(
        long,
        default_value = "default",
        help = "Progress bar style: default, detailed"
    )]
    pub style: ProgressBarStyle,

    #[arg(short, long, help = "Copy directories recursively")]
    pub recursive: bool,

    #[arg(
        short = 'j',
        default_value_t = 4,
        help = "Number of concurrent copy operations for multiple files"
    )]
    pub concurrency: usize,

    #[arg(
        short = 'c',
        long = "continue",
        help = "Continue copying by skipping files that are already complete"
    )]
    pub continue_copy: bool,

    #[arg(
        short = 'f',
        long,
        help = "if an existing destination file cannot be opened, remove it and try again"
    )]
    pub force: bool,

    #[arg(short = 'i', long, help = "prompt before overwrite")]
    pub interactive: bool,

    #[arg(long, help = "use full source file name under DIRECTORY")]
    pub parents: bool,

    #[arg(
        short = 'p',
        long = "preserve",
        value_name = "ATTR_LIST",
        default_missing_value = "",
        help = "preserve the specified attributes"
    )]
    pub preserve: Option<String>,

    #[arg(
        long = "attributes-only",
        help = "don't copy the file data, just the attributes"
    )]
    pub attributes_only: bool,

    #[arg(
        long = "remove-destination",
        help = "remove each existing destination file before attempting to open it"
    )]
    pub remove_destination: bool,

    #[arg(
            short = 's',
            long = "symbolic-link",
            value_name = "MODE",
            default_missing_value = "auto",
            num_args = 0..=1,
            help = "make symbolic links instead of copying (auto, absolute, or relative)"
        )]
    pub symbolic_link: Option<SymlinkMode>,

    #[arg(
        short = 'l',
        long = "link",
        help = "hard link files instead of copying"
    )]
    pub hard_link: bool,

    #[arg(
        short = 'P',
        long = "no-dereference",
        help = "never follow symbolic links in SOURCE"
    )]
    pub no_dereference: bool,
    #[arg(
        short = 'L',
        long = "dereference",
        help = "always follow symbolic links in SOURCE"
    )]
    pub dereference: bool,

    #[arg(
        short = 'H',
        long = "dereference-command-line",
        help = "follow symbolic links only on command line"
    )]
    pub dereference_command_line: bool,

    #[arg(
        short = 'b',
        long = "backup",
        value_name = "CONTROL",
        default_missing_value = "existing",
        num_args = 0..=1,
        help = "make a backup of each existing destination file (none, numbered, existing, simple)"
    )]
    pub backup: Option<BackupMode>,

    #[arg(
        long = "reflink",
        value_name = "WHEN",
        default_missing_value = "auto",
        num_args = 0..=1,
        help = "control clone/CoW copies (auto, always, never)"
    )]
    pub reflink: Option<ReflinkMode>,
}

#[derive(Debug, Clone, Copy)]
pub struct CopyOptions {
    pub recursive: bool,
    pub concurrency: usize,
    pub resume: bool,
    pub force: bool,
    pub interactive: bool,
    pub parents: bool,
    pub preserve: PreserveAttr,
    pub attributes_only: bool,
    pub remove_destination: bool,
    pub symbolic_link: Option<SymlinkMode>,
    pub hard_link: bool,
    pub follow_symlink: FollowSymlink,
    pub style: ProgressBarStyle,
    pub backup: Option<BackupMode>,
    pub reflink: Option<ReflinkMode>,
}

impl CopyOptions {
    pub fn none() -> Self {
        Self {
            recursive: false,
            concurrency: 4,
            resume: false,
            force: false,
            interactive: false,
            parents: false,
            preserve: PreserveAttr::none(),
            attributes_only: false,
            remove_destination: false,
            symbolic_link: None,
            hard_link: false,
            follow_symlink: FollowSymlink::NoDereference,
            style: ProgressBarStyle::Default,
            backup: None,
            reflink: None,
        }
    }
}

impl From<&CLIArgs> for CopyOptions {
    fn from(cli: &CLIArgs) -> Self {
        Self {
            recursive: cli.recursive,
            concurrency: cli.concurrency,
            resume: cli.continue_copy,
            force: cli.force,
            interactive: cli.interactive,
            parents: cli.parents,
            preserve: match &cli.preserve {
                None => PreserveAttr::none(),
                Some(s) => {
                    PreserveAttr::from_string(s).expect("unable to parse preserve attribute")
                }
            },
            attributes_only: cli.attributes_only,
            remove_destination: cli.remove_destination,
            symbolic_link: cli.symbolic_link,
            hard_link: cli.hard_link,
            follow_symlink: FollowSymlink::NoDereference,
            style: cli.style,
            backup: cli.backup,
            reflink: cli.reflink,
        }
    }
}

impl CLIArgs {
    pub fn follow_symlink_mode(&self) -> Result<FollowSymlink, String> {
        match (
            self.no_dereference,
            self.dereference,
            self.dereference_command_line,
        ) {
            (true, false, false) => Ok(FollowSymlink::NoDereference),
            (false, true, false) => Ok(FollowSymlink::Dereference),
            (false, false, true) => Ok(FollowSymlink::CommandLineSymlink),
            (false, false, false) => Ok(FollowSymlink::NoDereference),
            _ => Err("only one of -P, -L, or -H may be specified".to_string()),
        }
    }
    pub fn validate(mut self) -> Result<(Vec<PathBuf>, PathBuf, CopyOptions), String> {
        let follow_symlink = self.follow_symlink_mode()?;
        let mut options = CopyOptions::from(&self);
        options.follow_symlink = follow_symlink;

        if options.reflink.is_some() {
            if options.hard_link {
                return Err("--reflink and --link cannot be used together".to_string());
            }
            if options.symbolic_link.is_some() {
                return Err("--reflink and --symbolic-link cannot be used together".to_string());
            }
        }

        if options.symbolic_link.is_some() {
            if options.hard_link {
                return Err("--symbolic-link and --link cannot be used together".to_string());
            }
            if options.resume {
                return Err("--symbolic-link and --continue cannot be used together".to_string());
            }
            if options.attributes_only {
                return Err(
                    "--symbolic-link and --attributes-only cannot be used together".to_string(),
                );
            }
        }

        if options.hard_link {
            if options.resume {
                return Err("--link and --continue cannot be used together".to_string());
            }
            if options.attributes_only {
                return Err("--link and --attributes-only cannot be used together".to_string());
            }
        }
        if options.attributes_only {
            options.preserve = PreserveAttr::all();
        }
        let (sources, destination) = if let Some(target) = self.target_directory {
            self.sources.push(self.destination);
            (self.sources, target)
        } else {
            (self.sources, self.destination)
        };

        Ok((sources, destination, options))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_symlink_and_hardlink_conflict() {
        let args = CLIArgs {
            sources: vec![PathBuf::from("source.txt")],
            destination: PathBuf::from("dest.txt"),
            target_directory: None,
            style: ProgressBarStyle::Default,
            recursive: false,
            concurrency: 4,
            continue_copy: false,
            force: false,
            interactive: false,
            parents: false,
            preserve: None,
            attributes_only: false,
            remove_destination: false,
            symbolic_link: Some(SymlinkMode::Auto),
            hard_link: true,
            dereference: true,
            no_dereference: false,
            dereference_command_line: false,
            backup: None,
            reflink: None,
        };

        let result = args.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("symbolic-link"));
    }

    #[test]
    fn test_validate_symlink_and_resume_conflict() {
        let args = CLIArgs {
            sources: vec![PathBuf::from("source.txt")],
            destination: PathBuf::from("dest.txt"),
            target_directory: None,
            style: ProgressBarStyle::Default,
            recursive: false,
            concurrency: 4,
            continue_copy: true,
            force: false,
            interactive: false,
            parents: false,
            preserve: None,
            attributes_only: false,
            remove_destination: false,
            symbolic_link: Some(SymlinkMode::Auto),
            hard_link: false,
            dereference: true,
            no_dereference: false,
            dereference_command_line: false,
            backup: None,
            reflink: None,
        };

        let result = args.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("continue"));
    }

    #[test]
    fn test_validate_hardlink_and_resume_conflict() {
        let args = CLIArgs {
            sources: vec![PathBuf::from("source.txt")],
            destination: PathBuf::from("dest.txt"),
            target_directory: None,
            style: ProgressBarStyle::Default,
            recursive: false,
            concurrency: 4,
            continue_copy: true,
            force: false,
            interactive: false,
            parents: false,
            preserve: None,
            attributes_only: false,
            remove_destination: false,
            symbolic_link: None,
            hard_link: true,
            dereference: true,
            no_dereference: false,
            dereference_command_line: false,
            backup: None,
            reflink: None,
        };

        let result = args.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("link"));
    }

    #[test]
    fn test_validate_success() {
        let args = CLIArgs {
            sources: vec![PathBuf::from("source.txt")],
            destination: PathBuf::from("dest.txt"),
            target_directory: None,
            style: ProgressBarStyle::Default,
            recursive: false,
            concurrency: 4,
            continue_copy: false,
            force: false,
            interactive: false,
            parents: false,
            preserve: None,
            attributes_only: false,
            remove_destination: false,
            symbolic_link: None,
            hard_link: false,
            dereference: true,
            no_dereference: false,
            dereference_command_line: false,
            backup: None,
            reflink: None,
        };

        let result = args.validate();
        assert!(result.is_ok());
    }
}
