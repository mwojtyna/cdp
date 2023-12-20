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
    case_sensitive: bool,
}

impl Tui {
    const BOTTOM_BAR_HEIGHT: u16 = 2;

    pub fn new(terminal: Terminal, case_sensitive: bool) -> Self {
        Self {
            terminal,
            case_sensitive,
        }
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

    fn get_path_line<'a>(
        path: &'a str,
        filter: &str,
        index: usize,
        scroll_offset: usize,
        height: usize,
        case_sensitive: bool,
    ) -> Line<'a> {
        let dimmed = Style::default().dim();

        if index > height.saturating_sub(Self::BOTTOM_BAR_HEIGHT as usize) + scroll_offset
            || index < scroll_offset
        {
            return Line::from(Span::styled(path, dimmed));
        }

        let mut l = 0;
        let mut r = 0;
        let mut largest_diff_l = 0;
        let mut largest_diff_r = 0;

        let path_cased = if case_sensitive {
            path.to_owned()
        } else {
            path.to_lowercase()
        };
        let filter_cased = if case_sensitive {
            filter.to_owned()
        } else {
            filter.to_lowercase()
        };

        while r <= path.len() {
            if r == path_cased.len() || path_cased.chars().nth(r) != filter_cased.chars().nth(r - l)
            {
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
            Span::styled(&path[..largest_diff_l], dimmed),
            Span::styled(
                &path[largest_diff_l..largest_diff_r],
                Style::default().fg(Color::Red).dim(),
            ),
            Span::styled(&path[largest_diff_r..], dimmed),
        ])
    }

    pub fn update(&mut self, app: &mut App) -> Result<()> {
        self.terminal.draw(|f| {
            app.filter();

            let list_items: Vec<ListItem> = app
                .filtered_dirs
                .iter()
                .enumerate()
                .map(|(i, dir)| {
                    ListItem::new(Self::get_path_line(
                        dir,
                        app.input_state.value(),
                        i,
                        app.list_state.offset(),
                        f.size().height as usize,
                        self.case_sensitive,
                    ))
                })
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

            let layout = Layout::new(
                Direction::Vertical,
                [
                    Constraint::Max(
                        f.size()
                            .height
                            .saturating_sub(list_items_len as u16)
                            .saturating_sub(Self::BOTTOM_BAR_HEIGHT),
                    ),
                    Constraint::Min(0),
                    Constraint::Max(1),
                    Constraint::Max(1),
                ],
            )
            .split(f.size());

            f.render_stateful_widget(list, layout[1], &mut app.list_state);
            f.render_widget(count, layout[2]);
            f.render_widget(input, layout[3]);

            f.set_cursor(
                layout[3].x.saturating_add(
                    Self::BOTTOM_BAR_HEIGHT.saturating_add(app.input_state.visual_cursor() as u16),
                ),
                layout[3].y,
            );
        })?;
        Ok(())
    }
}
