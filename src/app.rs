use ratatui::widgets::ListState;
use std::{fs, path::PathBuf};

#[derive(Default)]
pub struct App {
    pub dirs: Vec<String>,
    pub list_state: ListState,
    pub should_quit: bool,
    pub submitted: bool,

    root_dir: PathBuf,
    stopper: PathBuf,
}

impl App {
    pub fn new(root_dir: PathBuf, stopper: PathBuf) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            list_state,
            root_dir,
            stopper,
            ..Self::default()
        }
    }
    pub fn find_projects(&mut self) {
        self.dirs = self.get_dirs(self.root_dir.clone());
    }

    pub fn next(&mut self) {
        self.list_state.select(Some(
            (self.list_state.selected().unwrap() + 1) % self.dirs.len(),
        ))
    }
    pub fn prev(&mut self) {
        let mut selected = self.list_state.selected().unwrap();
        if selected as isize - 1 < 0 {
            selected = self.dirs.len() - 1;
        } else {
            selected -= 1;
        }

        self.list_state.select(Some(selected))
    }
    pub fn get_selected(&self) -> &String {
        &self.dirs[self.list_state.selected().unwrap()]
    }

    pub fn submit(&mut self) {
        self.submitted = true;
        self.should_quit = true;
    }
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    // TODO: Parallelize
    fn get_dirs(&self, current: PathBuf) -> Vec<String> {
        if let Ok(entries) = fs::read_dir(&current) {
            let dirs: Vec<PathBuf> = entries
                .filter_map(|entry| entry.ok().map(|entry| entry.path()))
                .filter(|entry| entry.is_dir())
                .collect();

            if dirs
                .iter()
                // It's safe to unwrap because only a file can have a name of '..'
                // https://doc.rust-lang.org/std/path/struct.Path.html#method.file_name
                .any(|dir| dir.file_name().expect("This should never happen") == self.stopper)
            {
                // If the current dir has STOPPER, add it
                let path_string = current.to_string_lossy().to_string();
                vec![path_string]
            } else {
                // If the current dir doesn't have STOPPER, add all its children who have it
                dirs.into_iter()
                    .flat_map(|dir| self.get_dirs(dir))
                    .collect()
            }
        } else {
            vec![]
        }
    }
}
