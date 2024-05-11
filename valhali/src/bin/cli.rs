use avahi_zbus::ServerProxy;
use clap::{Parser, Subcommand};
use valhali::status::ServerStatus;
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
            let status = ServerStatus::from_server(&server).await?;
            println!("{status}")
        }
    }

    Ok(())
}
