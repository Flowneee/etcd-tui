use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// List of etcd endpoints
    #[arg(
        short,
        long,
        num_args(0..),
        value_delimiter = ',',
        default_values_t = vec!["127.0.0.1:2379".to_string()]
    )]
    pub endpoints: Vec<String>,

    /// User name (requires password)
    #[arg(short, long, requires = "password")]
    pub user: Option<String>,
    /// User password
    #[arg(short, long, env = "ETCD_PASSWORD")]
    pub password: Option<String>,
}

impl Cli {
    pub fn credentials(&self) -> Option<(String, String)> {
        self.user
            .clone()
            .and_then(|user| self.password.clone().map(|password| (user, password)))
    }
}
