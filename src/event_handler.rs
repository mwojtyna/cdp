use crate::app::App;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::time::Duration;

pub struct EventHandler {
    poll_timeout: Duration,
}

impl EventHandler {
    pub fn new(poll_timeout: Duration) -> Self {
        Self { poll_timeout }
    }

    pub fn handle(&self, app: &mut App) -> Result<()> {
        if event::poll(self.poll_timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('j') => {
                            app.next();
                        }
                        KeyCode::Char('k') => {
                            app.prev();
                        }
                        KeyCode::Char('q') => {
                            app.quit();
                        }
                        KeyCode::Enter => {
                            app.submit();
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }
}
