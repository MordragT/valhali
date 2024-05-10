use std::fmt;

use avahi_zbus::ServerProxy;
use clap::{Parser, Subcommand};
use zbus::Connection;

#[derive(Parser)]
struct App {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Resolve { domain: String },
    Service { service: String },
    Discover,
    Status,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::parse();

    let connection = Connection::system().await?;
    let server = ServerProxy::new(&connection).await?;

    match app.cmd {
        Cmd::Resolve { domain } => todo!(),
        Cmd::Service { service } => todo!(),
        Cmd::Discover => todo!(),
        Cmd::Status => {
            let status = Status::from_server(&server).await?;
            println!("{status}")
        }
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Status {
    host_name: String,
    domain_name: String,
    version: String,
    api: u32,
}

impl Status {
    pub async fn from_server(server: &ServerProxy<'_>) -> zbus::Result<Self> {
        let host_name = server.get_host_name().await?;
        let domain_name = server.get_domain_name().await?;
        let version = server.get_version_string().await?;
        let api = server.get_api_version().await?;

        Ok(Self {
            host_name,
            domain_name,
            version,
            api,
        })
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            host_name,
            domain_name,
            version,
            api,
        } = self;

        write!(
            f,
            "Host: {host_name}\nDomain: {domain_name}\nVersion: {version}\nApi: {api}"
        )
    }
}
