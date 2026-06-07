mod config;

use clap::{Parser, Subcommand, ValueEnum};
use url::Url;
use relay_common::model::relay::RelayType;

#[derive(Parser, Debug)]
#[command(name = "relay")]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum RelayTypeArg {
    Http,
    Tcp,
    Udp,
    Ws,
}

impl From<RelayTypeArg> for RelayType {
    fn from(value: RelayTypeArg) -> Self {
        match value {
            RelayTypeArg::Http => RelayType::Http,
            RelayTypeArg::Tcp => RelayType::Tcp,
            RelayTypeArg::Udp => RelayType::Udp,
            RelayTypeArg::Ws => RelayType::Ws,
        }
    }
}

#[derive(Subcommand, Debug)]
enum Commands {
    Login {
        server: Option<Url>
    },
    Run {
        /// Not everybody can select the port they want and also it never needs to be required
        port: Option<u16>,

        /// Needed for the relay to know if we can f.e. use basic auth for http
        #[arg(long, value_enum)]
        host_type: RelayTypeArg,
    }
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Login { .. } => todo!(),
        Commands::Run { .. } => todo!(),
    }
}
