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
    // TODO: Color the part of each path which matches the filter
    pub fn update(&mut self, app: &mut App) -> Result<()> {
        self.terminal.draw(|f| {
            app.filter();

            let list_items: Vec<ListItem> = app
                .filtered_dirs
                .iter()
                .map(|dir| ListItem::new(dir.as_str()))
                .collect();
            let list_items_len = list_items.len();

            let list = List::new(list_items)
                .highlight_style(Style::default().bg(Color::White).fg(Color::Black).bold())
                .highlight_symbol("> ");

            let count = Paragraph::new(format!("  {}/{}", app.filtered_dirs.len(), app.dirs.len()))
                .fg(Color::from_str("#AAAAAA").unwrap());

            let input = Paragraph::new("> ".to_owned() + app.input_state.value()).bold();

            let layout = Layout::new()
                .constraints([
                    Constraint::Max(
                        f.size()
                            .height
                            .saturating_sub(list_items_len as u16)
                            .saturating_sub(2),
                    ),
                    Constraint::Min(0),
                    Constraint::Max(1),
                    Constraint::Max(1),
                ])
                .split(f.size());

            f.render_stateful_widget(list, layout[1], &mut app.list_state);
            f.render_widget(count, layout[2]);
            f.render_widget(input, layout[3]);

            f.set_cursor(
                layout[3]
                    .x
                    .saturating_add(2_u16.saturating_add(app.input_state.visual_cursor() as u16)),
                layout[3].y,
            );
        })?;
        Ok(())
    }
}
