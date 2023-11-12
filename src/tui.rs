use crate::app::App;
use anyhow::Result;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use std::{
    io::{stderr, Stderr},
    str::FromStr,
};

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
        Ok(())
    }
    pub fn close(&self) -> Result<()> {
        stderr().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    // TODO: Customizable colors
    pub fn update(&mut self, app: &mut App) -> Result<()> {
        self.terminal.draw(|f| {
            app.filter();

            let layout = Layout::new()
                .constraints([Constraint::Min(0), Constraint::Max(1), Constraint::Max(1)])
                .split(f.size());

            let list_items: Vec<ListItem> = app
                .filtered_dirs
                .iter()
                .map(|dir| ListItem::new(dir.as_str()))
                .collect();
            let list = List::new(list_items)
                .highlight_style(Style::default().bg(Color::White).fg(Color::Black).bold());

            let count = Paragraph::new(format!("{}/{}", app.filtered_dirs.len(), app.dirs.len()))
                .fg(Color::from_str("#AAAAAA").unwrap());

            let input = Paragraph::new(app.input_state.value());

            f.render_stateful_widget(list, layout[0], &mut app.list_state);
            f.render_widget(count, layout[1]);
            f.render_widget(input, layout[2]);

            f.set_cursor(
                layout[2]
                    .x
                    .saturating_add(app.input_state.visual_cursor() as u16),
                layout[2].y,
            );
        })?;
        Ok(())
    }
}
