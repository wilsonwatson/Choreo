use std::path::PathBuf;

use clap::Parser;


#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// Disables the GUI.
    #[arg(long)]
    pub background: bool,
    /// Files to generate.
    pub files: Vec<PathBuf>,
}
