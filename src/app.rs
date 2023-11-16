use crate::cli::Args;
use ignore::{overrides::OverrideBuilder, WalkBuilder, WalkState};
use ratatui::widgets::ListState;
use std::sync::{Arc, Mutex};
use tui_input::Input;

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
        self.dirs = self.walk_dirs();
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

        let last = self.filtered_dirs.len().saturating_sub(1);
        if self
            .list_state
            .selected()
            .expect("Nothing is selected. This should never happen.")
            > last
        {
            self.list_state.select(Some(last));
        }
    }
    pub fn get_selected(&self) -> &String {
        &self.filtered_dirs[self
            .list_state
            .selected()
            .expect("Nothing is selected. This should never happen.")]
    }

    pub fn next(&mut self) {
        if self.filtered_dirs.is_empty() {
            return;
        }

        self.list_state.select(Some(
            (self
                .list_state
                .selected()
                .expect("Nothing is selected. This should never happen.")
                + 1)
                % self.filtered_dirs.len(),
        ))
    }
    pub fn prev(&mut self) {
        if self.filtered_dirs.is_empty() {
            return;
        }

        let mut selected = self
            .list_state
            .selected()
            .expect("Nothing is selected. This should never happen.");
        if selected as isize - 1 < 0 {
            selected = self.filtered_dirs.len() - 1;
        } else {
            selected -= 1;
        }

        self.list_state.select(Some(selected))
    }
    pub fn first(&mut self) {
        if self.filtered_dirs.is_empty() {
            return;
        }

        self.list_state.select(Some(0))
    }
    pub fn last(&mut self) {
        if self.filtered_dirs.is_empty() {
            return;
        }

        self.list_state
            .select(Some(self.filtered_dirs.len().saturating_sub(1)))
    }

    pub fn submit(&mut self) {
        if self.filtered_dirs.is_empty() {
            return;
        }
        self.submitted = true;
        self.should_quit = true;
    }
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    fn walk_dirs(&self) -> Vec<String> {
        let walker = WalkBuilder::new(&self.config.root_dir)
            .hidden(false)
            .follow_links(true)
            .overrides(
                OverrideBuilder::new(&self.config.root_dir)
                    .add("!.git/")
                    .unwrap()
                    .add(&self.config.stopper)
                    .expect("Invalid stopper file name")
                    .build()
                    .unwrap(),
            )
            .threads(self.config.cpus)
            .build_parallel();

        let paths = Arc::new(Mutex::new(Vec::new()));
        walker.run(|| {
            Box::new(|entry| {
                if let Ok(path) = entry {
                    if path.file_name().to_string_lossy() == self.config.stopper {
                        if let Some(parent) = path.path().parent() {
                            paths
                                .lock()
                                .unwrap()
                                .push(parent.to_string_lossy().into_owned());
                        }
                    }
                    WalkState::Continue
                } else {
                    WalkState::Skip
                }
            })
        });

        let paths = paths.lock().unwrap().clone();
        paths
    }
}
