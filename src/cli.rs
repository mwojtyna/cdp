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

    #[arg(
        long,
        default_value_t = false,
        help = "Continue searching in a directory subtree when a stopper file is found"
    )]
    pub greedy: bool,

    #[arg(
        long,
        default_value_t = num_cpus::get() / 2,
        help = "Amount of logical cores to use for searching the root_dir, defaults to half available"
    )]
    pub cpus: usize,

    #[arg(long, default_value_t = false)]
    pub case_sensitive: bool,
}
