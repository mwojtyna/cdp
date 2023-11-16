use crate::app::App;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use std::time::Duration;
use tui_input::backend::crossterm::EventHandler as CrosstermEventHandler;

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
                    if key.modifiers.contains(KeyModifiers::CONTROL) {
                        match key.code {
                            KeyCode::Char('j') => app.next(),
                            KeyCode::Char('k') => app.prev(),
                            KeyCode::Char('c') => app.quit(),
                            KeyCode::Home => app.first(),
                            KeyCode::End => app.last(),
                            _ => {
                                app.input_state.handle_event(&Event::Key(key));
                            }
                        }

                        return Ok(());
                    }

                    match key.code {
                        KeyCode::Down => app.next(),
                        KeyCode::Up => app.prev(),
                        KeyCode::Esc => app.quit(),
                        KeyCode::Enter => app.submit(),
                        _ => {
                            app.input_state.handle_event(&Event::Key(key));
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
