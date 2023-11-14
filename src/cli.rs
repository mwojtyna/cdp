use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    pub dir: PathBuf,

    #[arg(long, default_value_t = String::from(".git"))]
    pub stopper: String,
}
