use ignore::{overrides::OverrideBuilder, WalkBuilder};
use ratatui::widgets::ListState;
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
        if self.dirs.is_empty() {
            return;
        }

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
        if self.dirs.is_empty() {
            return;
        }

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

    fn walk_dirs(&self) -> Vec<String> {
        let walk = WalkBuilder::new(&self.config.root_dir)
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
            .build();

        let mut out = Vec::new();
        for dir in walk.flatten() {
            if dir.file_name().to_string_lossy() == self.config.stopper {
                out.push(dir.path().parent().unwrap().to_string_lossy().into_owned());
            }
        }
        out
    }
}
