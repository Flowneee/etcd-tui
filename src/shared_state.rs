use anyhow::{bail, Result};
use etcd_client::{Client, ConnectOptions, GetOptions};
use tokio::sync::mpsc::UnboundedSender;

use crate::{cli::Cli, events::Event};

#[derive(Clone)]
pub struct SharedState {
    etcd_client: Client,
    event_tx: UnboundedSender<Event>,
}

impl SharedState {
    pub async fn new(cli: Cli, event_tx: UnboundedSender<Event>) -> Result<Self> {
        let mut client_conn_opts = ConnectOptions::new();
        if let Some((user, password)) = cli.credentials() {
            client_conn_opts = client_conn_opts.with_user(user, password);
        }
        Ok(Self {
            etcd_client: Client::connect(cli.endpoints, Some(client_conn_opts)).await?,
            event_tx,
        })
    }

    pub fn etcd_client(&self) -> Client {
        self.etcd_client.clone()
    }

    pub async fn load_keys(&self) -> Result<Vec<String>> {
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

    pub async fn get_key(&self, key: &str) -> Result<String> {
        let response = self.etcd_client().get(key, None).await?;

        match response.kvs().len() {
            1 => {}
            0 => bail!("Key not found"),
            _ => bail!("Multiple key values returned"),
        }
        // SAFETY: just checked length of kvs()
        Ok(response.kvs()[0].value_str()?.to_string())
    }

    pub async fn put_key(&self, key: &str, value: String) -> Result<()> {
        let _ = self.etcd_client().put(key, value, None).await?;
        Ok(())
    }

    pub async fn delete_key(&self, key: &str) -> Result<()> {
        let _ = self.etcd_client().delete(key, None).await?;
        Ok(())
    }

    pub fn send_event(&self, event: Event) -> Result<()> {
        Ok(self.event_tx.send(event)?)
    }

    pub fn tick(&self) -> Result<()> {
        self.send_event(Event::Tick)
    }
}
