use crate::app::App;
use anyhow::Result;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::CrosstermBackend, style::*, widgets::*};
use std::io::{stderr, Stderr};

type Terminal = ratatui::Terminal<CrosstermBackend<Stderr>>;

pub struct Tui {
    terminal: Terminal,
}

impl Tui {
    pub fn new(terminal: Terminal) -> Self {
        Self { terminal }
    }

    pub fn open(&mut self) -> Result<()> {
        stderr().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        self.terminal.clear()?;
        self.terminal.hide_cursor()?;
        Ok(())
    }
    pub fn close(&self) -> Result<()> {
        stderr().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn update(&mut self, app: &mut App) -> Result<()> {
        self.terminal.draw(|f| {
            let dirs: Vec<ListItem> = app
                .dirs
                .iter()
                .map(|dir| ListItem::new(dir.as_str()))
                .collect();
            let list =
                List::new(dirs).highlight_style(Style::default().bg(Color::White).fg(Color::Black));

            f.render_stateful_widget(list, f.size(), &mut app.list_state);
        })?;
        Ok(())
    }
}
