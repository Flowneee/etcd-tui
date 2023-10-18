use std::panic;

use anyhow::Result;
use clap::Parser;
use futures::TryFutureExt;
use tokio::{pin, spawn, sync::mpsc::unbounded_channel};

use crate::{
    app::App, cli::Cli, components::Component, events::Event, shared_state::SharedState, tui::Tui,
};

#[macro_use]
mod events;

mod app;
mod cli;
mod components;
mod shared_state;
mod tui;
mod ui;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let (event_tx, mut event_rx) = unbounded_channel();
    let shared_state = SharedState::new(cli, event_tx).await?;
    let mut app = App::new(shared_state.clone());

    let mut tui = Tui::new()?;
    tui.enter()?;

    let event_handler = spawn(events::event_handler(shared_state.clone()))
        .err_into()
        .and_then(|x| async { x });
    pin!(event_handler);

    while !app.should_quit() {
        tui.terminal_mut().draw(|f| app.draw(f, f.size()))?;

        let event = tokio::select! {
            event = event_rx.recv() => {
                event.unwrap_or_else(|| Event::Quit(Ok(())))
            },
            eh = &mut event_handler => {
                Event::Quit(eh)
            }
        };

        if let Err(err) = app.handle_event(event) {
            app.quit(Err(err));
        }
    }

    tui.exit()?;
    app.take_result()
}
