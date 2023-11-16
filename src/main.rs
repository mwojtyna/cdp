use anyhow::Result;
use app::App;
use clap::Parser;
use cli::Args;
use event_handler::EventHandler;
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::{io::stderr, time::Duration};
use tui::Tui;

mod app;
mod cli;
mod event_handler;
mod tui;

fn main() -> Result<()> {
    let args = Args::parse();

    let terminal = Terminal::new(CrosstermBackend::new(stderr()))?;
    let mut tui = Tui::new(terminal);
    tui.open()?;

    let mut app = App::new(args);
    app.find_projects();

    let event_handler = EventHandler::new(Duration::from_millis(250));

    while !app.should_quit {
        tui.update(&mut app)?;
        event_handler.handle(&mut app)?;
    }

    tui.close()?;
    if app.submitted {
        print!("{}", app.get_selected());
    }

    Ok(())
}
