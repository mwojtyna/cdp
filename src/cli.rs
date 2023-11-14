use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Default)]
#[command(version)]
pub struct Args {
    pub dir: PathBuf,

    #[arg(
        long,
        default_value = ".git",
        help = "List directories containing this file"
    )]
    pub stopper: String,

    #[arg(
        long,
        num_args = 0..,
        value_delimiter = ',',
        default_value = "node_modules,vendor,target,build,dist,.git,.cache,.next",
        help = "Ignore these directories"
    )]
    pub ignore: Vec<String>,
}
