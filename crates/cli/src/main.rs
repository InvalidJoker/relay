mod auth;
mod client;
mod config;

use crate::auth::TokenResponse;
use crate::client::Client;
use anyhow::Context;
use clap::{Parser, Subcommand};
use relay_common::model::{
    HostConfig, RelayType,
    http::{AuthConfig, HostConfig as HttpHostConfig},
    tcp::HostConfig as TcpHostConfig,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;
use std::time::Duration;
use tracing::{error, info};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use url::Url;

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
        server: Option<Url>,
    },
    Http {
        /// The Local port to listen on
        port: u16,

        /// Not everybody can select the subdomain they want and also it never needs to be required
        #[arg(short, long)]
        subdomain: Option<String>,

        #[arg(long, requires = "password")]
        username: Option<String>,

        #[arg(long, requires = "username")]
        password: Option<String>,

        #[arg(long)]
        save: bool,
    },
    Tcp {
        /// The Local port to listen on
        port: u16,

        /// Not everybody can select the port they want and also it never needs to be required
        remote_port: Option<u16>,

        #[arg(long)]
        save: bool,
    },
    Run {
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Config {
    server: Url,
    secret: String,
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
        secret: String::new(),
    }); // the default config is only used for login, so it's fine to have an empty secret

    let client = reqwest::Client::new();

    match args.command {
        Commands::Login { server } => {
            let server =
                server.unwrap_or_else(|| Url::parse("https://relay.invalidjoker.dev").unwrap());

            let response = auth::request_device_code(server.clone(), &client)
                .await
                .context("Failed to request device code")?;

            info!(
                "Please visit {} to login with the code: {}",
                response.verification_uri_complete, response.user_code
            );

            loop {
                tokio::time::sleep(Duration::from_secs(response.interval.unwrap_or(5))).await;

                let token = client
                    //.post("https://auth.example.com/api/auth/device/token")
                    .post(server.join("/api/auth/device/token")?)
                    .json(&serde_json::json!({
                        "grant_type": "urn:ietf:params:oauth:grant-type:device_code",
                        "device_code": response.device_code,
                        "client_id": "cli"
                    }))
                    .send()
                    .await?
                    .json::<TokenResponse>()
                    .await?;

                if let Some(access_token) = token.access_token {
                    let config = Config {
                        server: server.clone(),
                        secret: access_token,
                    };

                    let config_path = config_path
                        .as_ref()
                        .expect("Failed to determine config path");

                    std::fs::create_dir_all(
                        config_path.parent().expect("Config path has no parent"),
                    )?;

                    std::fs::write(config_path, toml::to_string(&config)?)?;

                    info!("Login successful");
                    break;
                }

                match token.error.as_deref() {
                    Some("authorization_pending") => continue,
                    Some("slow_down") => continue,
                    Some("access_denied") => {
                        eprintln!("Login denied");
                        break;
                    }
                    Some("expired_token") => {
                        eprintln!("Login expired");
                        break;
                    }
                    Some("invalid_grant") => {
                        eprintln!("Invalid device code");
                        break;
                    }
                    _ => {}
                }
            }
        }
        Commands::Http {
            port,
            subdomain,
            username,
            password,
            save
        } => {
            let relay_info = auth::get_relay_info(config.server.clone(), &client)
                .await
                .context("Failed to get relay info")?;

            let auth = match (username, password) {
                (Some(u), Some(p)) => Some(AuthConfig {
                    username: u,
                    password: p,
                }),
                _ => None,
            };

            let host_config = HostConfig::Http(HttpHostConfig {
                local_port: port,
                domain: subdomain.clone(),
                auth: auth.clone(),
            });

            let client = Client::new(
                "localhost",
                port,
                relay_info.relay_url,
                host_config,
                config.secret,
            )
            .await?;

            if save {
                let local_config = config::Config {
                    relay_type: RelayType::Http,
                    port,
                    remote_port: None,
                    domain: subdomain,
                    auth
                };

                let path = PathBuf::from("relay.toml");
                std::fs::write(path, toml::to_string(&local_config)?)?;
                info!("Config saved to relay.toml");
            }

            client.listen().await?;
        }
        Commands::Tcp { port, remote_port, save } => {
            let relay_info = auth::get_relay_info(config.server.clone(), &client)
                .await
                .context("Failed to get relay info")?;

            let host_config = HostConfig::Tcp(TcpHostConfig {
                local_port: port,
                remote_port,
            });

            let client = Client::new(
                "localhost",
                port,
                relay_info.relay_url,
                host_config,
                config.secret,
            )
            .await?;

            if save {
                let local_config = config::Config {
                    relay_type: RelayType::Tcp,
                    port,
                    remote_port,
                    domain: None,
                    auth: None,
                };

                let path = PathBuf::from("relay.toml");
                std::fs::write(path, toml::to_string(&local_config)?)?;
                info!("Config saved to relay.toml");
            }

            client.listen().await?;
        }
        Commands::Run { path } => {
            let path = path.unwrap_or_else(|| PathBuf::from("relay.toml"));

            let config_content = std::fs::read_to_string(&path)
                .with_context(|| format!("Failed to read config file: {:?}", path))?;

            let local_config: config::Config = toml::from_str(&config_content)
                .context("Failed to parse config file")?;

            let relay_info = auth::get_relay_info(config.server.clone(), &client)
                .await
                .context("Failed to get relay info")?;

            match local_config.relay_type {
                RelayType::Tcp => {
                    info!("Running TCP relay with config: {:?}", local_config);

                    let host_config = HostConfig::Tcp(TcpHostConfig {
                        local_port: local_config.port,
                        remote_port: local_config.remote_port,
                    });

                    let client = Client::new(
                        "localhost",
                        local_config.port,
                        relay_info.relay_url,
                        host_config,
                        config.secret,
                    )
                        .await?;
                    client.listen().await?;
                }
                RelayType::Http => {
                    info!("Running HTTP relay with config: {:?}", local_config);

                    let host_config = HostConfig::Http(HttpHostConfig {
                        local_port: local_config.port,
                        domain: local_config.domain,
                        auth: local_config.auth,
                    });

                    let client = Client::new(
                        "localhost",
                        local_config.port,
                        relay_info.relay_url,
                        host_config,
                        config.secret,
                    )
                        .await?;
                    client.listen().await?;
                }
            }
        }
    }

    Ok(())
}
