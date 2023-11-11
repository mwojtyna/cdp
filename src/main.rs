use log::error;
use std::{fs, path::PathBuf, str::FromStr};

// TODO: TUI
// TODO: parallelize

// TODO: Configurable
const ROOT_DIR: &str = "/home/mati/developer/";
const STOPPER: &str = ".git";

fn main() {
    env_logger::builder()
        .format_timestamp(None)
        .format_target(false)
        .init();

    let start_dir = PathBuf::from_str(ROOT_DIR).expect("Could not open root directory");
    if let Err(err) = start_dir.read_dir() {
        error!("Error reading '{}': {}", start_dir.display(), err);
        return;
    }

    let dirs = get_dirs(start_dir);
    dbg!(dirs);
}

fn get_dirs(current: PathBuf) -> Vec<PathBuf> {
    if let Ok(entries) = fs::read_dir(&current) {
        let dirs: Vec<PathBuf> = entries
            .filter_map(|entry| entry.ok().map(|entry| entry.path()))
            .filter(|entry| entry.is_dir())
            .collect();

        if dirs
            .iter()
            // It's safe to unwrap because only a file can have a name of '..'
            // https://doc.rust-lang.org/std/path/struct.Path.html#method.file_name
            .any(|dir| dir.file_name().expect("This should never happen") == STOPPER)
        {
            // If the current dir has STOPPER, add it
            vec![current]
        } else {
            // If the current dir doesn't have STOPPER, add all its children who have it
            dirs.into_iter().flat_map(get_dirs).collect()
        }
    } else {
        vec![]
    }
}
