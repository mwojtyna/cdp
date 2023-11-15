use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Default)]
#[command(version)]
pub struct Args {
    pub root_dir: PathBuf,

    #[arg(
        long,
        default_value = ".git",
        help = "Search for directories containing this file"
    )]
    pub stopper: String,
}
