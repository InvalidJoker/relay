mod config;
mod client;

use std::env;
use std::path::PathBuf;
use anyhow::Context;
use clap::{Parser, Subcommand};
use rand::RngExt;
use tracing::{error, info};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use url::Url;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use tracing_subscriber::fmt::writer::MakeWriterExt;
use crate::client::Client;

#[derive(Parser, Debug)]
#[command(name = "relay")]
#[clap(author, version, about)]
struct Cli {
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Login {
        server: Option<Url>
    },
    Http {
        /// The Local port to listen on
        port: u16,

        /// Not everybody can select the subdomain they want and also it never needs to be required
        subdomain: Option<String>,
    },
    Tcp {
        /// The Local port to listen on
        port: u16,

        /// Not everybody can select the port they want and also it never needs to be required
        remote_port: Option<u16>,
    },
    // TODO: run command, run from config
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Config {
    server: Url,
    secret: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::LevelFilter::from_level(
            if cfg!(debug_assertions) || env::var("DEBUG").unwrap_or_default().eq("1") {
                tracing::Level::DEBUG
            } else {
                tracing::Level::INFO
            },
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args = Cli::parse();

    let config_path = args
        .config
        .clone()
        .or_else(|| dirs::home_dir().map(|d| d.join(".config/relay.toml")));


    let config = config_path
        .as_ref()
        .and_then(|path| std::fs::read_to_string(path).ok())
        .and_then(|content| toml::from_str(&content).ok());

    if config.is_none() && !matches!(args.command, Commands::Login { .. }) {
        error!("Config file not found, please login first with `relay login`");
        std::process::exit(1);
    }

    let config = config.unwrap_or_else(|| Config {
        server: Url::parse("https://relay.invalidjoker.dev").unwrap(),
        secret: None,
    }); // the default config is only used for login, so it's fine to have an empty secret

    match args.command {
        Commands::Login { server } => {
            let id = Uuid::new_v4().to_string();
            info!("Visit https://relay.local/login?token={id} to login");

            // empty lines
            println!();

            // write config
            let config = Config {
                server: server.unwrap_or_else(|| Url::parse("https://relay.invalidjoker.dev").unwrap()),
                secret: Some(id),
            };

            let config_path = config_path
                .as_ref()
                .expect("Failed to determine config path");

            std::fs::create_dir_all(
                config_path.parent().expect("Config path has no parent")
            )?;

            std::fs::write(
                config_path,
                toml::to_string(&config)?
            )?;

            info!("Login successful");
        }
        Commands::Http { port, subdomain } => {
            info!("Reaching out to relay on relay.invalidjoker.dev");

            let nato_alphabet = vec![
                "alpha", "bravo", "charlie", "delta", "echo", "foxtrot", "golf", "hotel", "india",
                "juliett", "kilo", "lima", "mike", "november", "oscar", "papa", "quebec",
                "romeo", "sierra", "tango", "uniform", "victor", "whiskey", "x-ray",
                "yankee", "zulu",
            ];

            let mut rng = rand::rng();


            let subdomain = subdomain.unwrap_or_else(|| {
                let mut subdomain = String::new();
                for _ in 0..3 {
                    let word = nato_alphabet[rng.random_range(0..nato_alphabet.len())];
                    subdomain.push_str(word);
                }
                subdomain
            });

            let domain = format!("{}.relay.invalidjoker.dev", subdomain);

            info!("Listening on port {port} with subdomain {domain}");

            // keep alive until ctrl+c
            loop {
                std::thread::park();
            }
        }
        Commands::Tcp { port, remote_port } => {
            info!("Reaching out to relay on relay.invalidjoker.dev");

            let local_host = "localhost";

            /*
            local_host: &str,
        local_port: u16,
        to: &str,
        secret: Option<&str>,
             */

            let client = Client::new(local_host,
                                     port,
                                     &config.server.host_str().unwrap(),
                                     config.secret.as_deref()
            ).await?;
            client.listen().await?;

            let remote_port = remote_port.unwrap_or_else(|| {
                let mut rng = rand::rng();
                rng.random_range(1024..65535)
            });

            info!("Listening on port {port} with remote port {remote_port}");

            // keep alive until ctrl+c
            loop {
                std::thread::park();
            }
        }
    }

    Ok(())
}
