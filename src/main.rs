use std::panic;

use anyhow::{bail, Result};

use clap::Parser;
use etcd_client::{Client, ConnectOptions, GetOptions};
use futures::TryFutureExt;

use tokio::{
    pin, spawn,
    sync::mpsc::{unbounded_channel, UnboundedSender},
};
use tui::Tui;

use crate::{app::App, cli::Cli, components::Component, events::Event};

mod app;
mod cli;
mod components;
mod events;
mod tui;
mod ui;
mod utils;

#[derive(Clone)]
pub struct SharedState {
    etcd_client: Client,
    event_tx: UnboundedSender<Event>,
}

impl SharedState {
    async fn new(cli: Cli, event_tx: UnboundedSender<Event>) -> Result<Self> {
        let mut client_conn_opts = ConnectOptions::new();
        if let Some((user, password)) = cli.credentials() {
            client_conn_opts = client_conn_opts.with_user(user, password);
        }
        Ok(Self {
            etcd_client: Client::connect(cli.endpoints, Some(client_conn_opts)).await?,
            event_tx,
        })
    }

    fn etcd_client(&self) -> Client {
        self.etcd_client.clone()
    }

    fn send_event(&self, event: Event) -> Result<()> {
        Ok(self.event_tx.send(event)?)
    }

    fn tick(&self) -> Result<()> {
        self.send_event(Event::Tick)
    }

    async fn load_keys(&self) -> Result<Vec<String>> {
        self.etcd_client()
            .get(
                vec![],
                Some(GetOptions::new().with_all_keys().with_keys_only()),
            )
            .await?
            .kvs()
            .iter()
            .map(|x| x.key_str().map(|x| x.to_string()))
            .collect::<Result<_, _>>()
            .map_err(Into::into)
    }

    async fn get_key(&self, key: &str) -> Result<String> {
        let response = self.etcd_client().get(key, None).await?;

        match response.kvs().len() {
            1 => {}
            0 => bail!("Key not found"),
            _ => bail!("Multiple key values returned"),
        }
        // SAFETY: just checked length of kvs()
        Ok(response.kvs()[0].value_str()?.to_string())
    }

    async fn put_key(&self, key: &str, value: String) -> Result<()> {
        let _ = self.etcd_client().put(key, value, None).await?;
        Ok(())
    }
}

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
