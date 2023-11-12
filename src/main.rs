use anyhow::Result;
use app::App;
use event_handler::EventHandler;
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::{io::stderr, path::PathBuf, str::FromStr, time::Duration};
use tui::Tui;

mod app;
mod event_handler;
mod tui;

// TODO: Configurable
const ROOT_DIR: &str = "/home/mati/developer/";
const STOPPER: &str = ".git";

fn main() -> Result<()> {
    env_logger::builder()
        .format_timestamp(None)
        .format_target(false)
        .init();

    let terminal = Terminal::new(CrosstermBackend::new(stderr()))?;
    let mut tui = Tui::new(terminal);
    tui.open()?;

    let mut app = App::new(PathBuf::from_str(ROOT_DIR)?, PathBuf::from_str(STOPPER)?);
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
