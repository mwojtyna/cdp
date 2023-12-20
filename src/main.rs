use anyhow::Result;
use app::App;
use clap::{error::ErrorKind, CommandFactory, Parser};
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

    // If the user passed a search query, don't open TUI, just print the first match
    if let Some(query) = args.clone().search_query {
        let mut app = App::new(args);
        app.find_projects();

        if app.select_first_match(query.as_str()) {
            print_result(app.get_selected());
            return Ok(());
        } else {
            let mut cmd = Args::command();
            cmd.error(ErrorKind::InvalidValue, "No match found for query")
                .exit()
        }
    }

    let terminal = Terminal::new(CrosstermBackend::new(stderr()))?;
    let mut tui = Tui::new(terminal, args.case_sensitive);
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
        print_result(app.get_selected());
    }

    Ok(())
}

fn print_result(value: &str) {
    println!("{}", value);
}
