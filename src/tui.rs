use crate::app::App;
use anyhow::Result;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use std::{
    cmp::min,
    io::{stderr, Stderr},
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

    pub fn update(&mut self, app: &mut App) -> Result<()> {
        fn get_path_line<'a>(path: &'a str, filter: &'a str) -> Line<'a> {
            let mut l = 0;
            let mut r = 0;
            let mut largest_diff_l = 0;
            let mut largest_diff_r = 0;

            while r <= path.len() {
                if r == path.len() || path.chars().nth(r) != filter.chars().nth(r - l) {
                    if r - l > largest_diff_r - largest_diff_l {
                        largest_diff_r = r;
                        largest_diff_l = l;
                    }
                    r += 1;
                    l = r;
                    continue;
                }
                r += 1;
            }

            Line::from(vec![
                Span::raw(&path[..largest_diff_l]).dim(),
                Span::styled(
                    &path[largest_diff_l..largest_diff_r],
                    Style::default().fg(Color::Red).dim(),
                ),
                Span::raw(&path[largest_diff_r..]).dim(),
            ])
        }

        self.terminal.draw(|f| {
            app.filter();

            let list_items: Vec<ListItem> = app
                .filtered_dirs
                .iter()
                .map(|dir| ListItem::new(get_path_line(dir, app.input_state.value())))
                .collect();
            let list_items_len = list_items.len();

            let list = List::new(list_items)
                .highlight_style(Style::default().bold().not_dim())
                .highlight_symbol("> ");

            let count = Paragraph::new(format!(
                "  {}/{}",
                min(
                    app.list_state
                        .selected()
                        .expect("Nothing is selected. This should never happen.")
                        + 1,
                    app.filtered_dirs.len()
                ),
                app.filtered_dirs.len()
            ))
            .fg(Color::White);

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
