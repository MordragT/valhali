use avahi_zbus::{EntryGroupProxy, EntryGroupState, ServerProxy, ServerState, Ttl};
use clap::Parser;
use std::{path::PathBuf, str::FromStr};
use tokio::{
    io,
    signal::unix::{signal, SignalKind},
};
use tracing::{debug, error, info, warn};
use valhali::{
    entry_group_add_record, entry_group_event_handler, name::NameBuf, rdata::Cname,
    server_event_handler,
};
use zbus::Connection;

#[derive(Parser)]
struct App {
    config: PathBuf,
}

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .try_init()?;

    // let app = App::parse();

    let connection = Connection::system().await?;
    let server = ServerProxy::new(&connection).await?;
    info!("Established connection to avahi dbus");

    server_event_handler(&server, |state, error| match state {
        ServerState::Failure | ServerState::Invalid => error!("{state:?}: {error}"),
        ServerState::Collision => warn!("{state:?}: {error}"),
        ServerState::Registering | ServerState::Running => info!("{state:?}: {error}"),
    })
    .await?;
    info!("Created server signals handler");

    let group_path = server.entry_group_new().await?;
    let group = EntryGroupProxy::new(&connection, group_path).await?;
    info!("Created new entry group");

    entry_group_event_handler(&group, |state, error| match state {
        EntryGroupState::Failure => error!("{state:?}: {error}"),
        EntryGroupState::Collision => warn!("{state:?}: {error}"),
        EntryGroupState::Established
        | EntryGroupState::Registering
        | EntryGroupState::Uncommitted => info!("{state:?}: {error}"),
    })
    .await?;
    info!("Created group signals handler");

    let cname = Cname::from_str(&server.get_host_name_fqdn().await?)?;
    entry_group_add_record(
        &group,
        &NameBuf::from_str("vault.local").unwrap(),
        Ttl::MINUTE,
        cname,
    )
    .await?;
    group.commit().await?;
    info!("Committed entry group");

    wait_for_shutdown().await?;
    info!("Shutting down");

    group.free().await?;

    Ok(())
}

async fn wait_for_shutdown() -> io::Result<()> {
    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;
    let mut sigquit = signal(SignalKind::quit())?;

    tokio::select! {
        _ = sigint.recv() => debug!("Received SIGINT"),
        _ = sigterm.recv() => debug!("Received SIGTERM"),
        _ = sigquit.recv() => debug!("Received SIGQUIT"),
    }

    Ok(())
}
