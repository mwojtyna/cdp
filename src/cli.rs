use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Default, Clone)]
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
        default_value_t = num_cpus::get() / 2,
        help = "Amount of logical cores to use for searching the root_dir, defaults to half available"
    )]
    pub cpus: usize,
}
