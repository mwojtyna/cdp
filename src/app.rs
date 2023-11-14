use ratatui::widgets::ListState;
use std::{collections::HashSet, fs, path::PathBuf};
use tui_input::Input;

use crate::cli::Args;

#[derive(Default)]
pub struct App {
    pub filtered_dirs: Vec<String>,
    pub should_quit: bool,
    pub submitted: bool,

    pub list_state: ListState,
    pub input_state: Input,

    dirs: Vec<String>,
    config: Args,
}

impl App {
    pub fn new(config: Args) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            list_state,
            config,
            ..Self::default()
        }
    }
    pub fn find_projects(&mut self) {
        self.dirs = self.get_dirs(&self.config.dir);
        self.dirs.sort();
        self.filtered_dirs = self.dirs.clone();
    }

    pub fn filter(&mut self) {
        self.filtered_dirs = self
            .dirs
            .iter()
            .filter(|dir| dir.contains(self.input_state.value()))
            .map(|dir| dir.to_string())
            .collect();

        let last_filtered = self.filtered_dirs.len().saturating_sub(1);
        if self
            .list_state
            .selected()
            .expect("Nothing is selected. This should never happen.")
            > last_filtered
        {
            self.list_state.select(Some(last_filtered));
        }
    }
    pub fn get_selected(&self) -> &String {
        &self.filtered_dirs[self
            .list_state
            .selected()
            .expect("Nothing is selected. This should never happen.")]
    }

    pub fn next(&mut self) {
        self.list_state.select(Some(
            (self
                .list_state
                .selected()
                .expect("Nothing is selected. This should never happen.")
                + 1)
                % self.dirs.len(),
        ))
    }
    pub fn prev(&mut self) {
        let mut selected = self
            .list_state
            .selected()
            .expect("Nothing is selected. This should never happen.");
        if selected as isize - 1 < 0 {
            selected = self.dirs.len() - 1;
        } else {
            selected -= 1;
        }

        self.list_state.select(Some(selected))
    }

    pub fn submit(&mut self) {
        self.submitted = true;
        self.should_quit = true;
    }
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    // TODO: https://docs.rs/ignore/latest/ignore/gitignore/struct.GitignoreBuilder.html
    fn get_dirs(&self, current: &PathBuf) -> Vec<String> {
        let mut skip_files = HashSet::<String>::from_iter(self.config.ignore.clone());
        skip_files.remove(&self.config.stopper);

        if let Ok(files) = fs::read_dir(current) {
            let files: Vec<PathBuf> = files
                .filter_map(|file| file.ok().map(|entry| entry.path()))
                .filter(|file| {
                    !skip_files.contains(
                        &file
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string(),
                    )
                })
                .collect();

            if files
                .iter()
                // It's safe to unwrap because only a file can have a name of '..'
                // https://doc.rust-lang.org/std/path/struct.Path.html#method.file_name
                .any(|file| {
                    file.file_name()
                        .expect("The directory name is invalid. This should never happen.")
                        .to_string_lossy()
                        == self.config.stopper
                })
            {
                // If the current dir has stopper, add it
                let path_string = current.to_string_lossy().to_string();
                vec![path_string]
            } else {
                // If the current dir doesn't have stopper, add all its children who have it
                files
                    .into_iter()
                    .flat_map(|dir| self.get_dirs(&dir))
                    .collect()
            }
        } else {
            vec![]
        }
    }
}
