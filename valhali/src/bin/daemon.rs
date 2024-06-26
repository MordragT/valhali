use avahi_zbus::{EntryGroupProxy, EntryGroupState, ServerProxy, ServerState, Ttl};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    str::FromStr,
    time::Duration,
};
use thiserror::Error;
use tokio::{
    fs, io,
    signal::unix::{signal, SignalKind},
    sync::watch,
    time,
};
use tracing::{debug, error, info, warn};
use valhali::{
    entry_group_add_record, entry_group_add_service, entry_group_event_handler,
    name::NameBuf,
    rdata::Cname,
    record::Record,
    server_event_handler, server_resolve_name,
    service::{Service, ServiceKind, TransportProtocol},
};
use zbus::Connection;

#[derive(Parser)]
struct App {
    config: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Config {
    aliases: Vec<NameBuf>,
    services: HashMap<String, ServiceConfig>,
}

impl Config {
    pub async fn from_file(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let contents = fs::read_to_string(path).await?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
struct ServiceConfig {
    alias: Option<NameBuf>,
    kind: ServiceKind,
    protocol: TransportProtocol,
    port: u16,
}

#[derive(Debug, Error)]
enum ConfigError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    TomlDe(#[from] toml::de::Error),
}

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .try_init()?;

    let app = App::parse();

    let mut config = Config::from_file(&app.config).await?;
    let (tx, mut rx) = watch::channel(config.clone());
    rx.mark_changed();
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(10));
        loop {
            interval.tick().await;

            match Config::from_file(&app.config).await {
                Ok(loaded_config) => {
                    if loaded_config != config {
                        tx.send(loaded_config.clone()).unwrap();
                        config = loaded_config;
                        debug!("Config changed");
                    }
                }
                Err(e) => error!("Config: {e}"),
            }
        }
    });
    info!("Created config file watcher");

    let connection = Connection::system().await?;
    let server = ServerProxy::new(&connection).await?;
    info!("Established connection to avahi dbus");

    server_event_handler(&server, |state, error| match state {
        ServerState::Failure | ServerState::Invalid => error!("Server {state:?}: {error}"),
        ServerState::Collision => warn!("Server {state:?}: {error}"),
        ServerState::Registering | ServerState::Running => info!("Server {state:?}: {error}"),
    })
    .await;
    info!("Created server signals handler");

    let group_path = server.entry_group_new().await?;
    let group = EntryGroupProxy::new(&connection, group_path).await?;
    info!("Created new entry group");

    entry_group_event_handler(&group, |state, error| match state {
        EntryGroupState::Failure => error!("EntryGroup {state:?}: {error}"),
        EntryGroupState::Collision => warn!("EntryGroup {state:?}: {error}"),
        EntryGroupState::Established
        | EntryGroupState::Registering
        | EntryGroupState::Uncommitted => info!("EntryGroup {state:?}: {error}"),
    })
    .await;
    info!("Created group signals handler");

    let cname = Cname::from_str(&server.get_host_name_fqdn().await?)?;
    let mut interval = time::interval(Duration::from_secs(1));

    loop {
        interval.tick().await;

        tokio::select! {
            _ = wait_for_shutdown() => {
                info!("Shutting down");
                group.free().await?;
                break;
            }
            _ = rx.changed() => {
                group.reset().await?;
                let config = rx.borrow_and_update();
                add_config(&server, &group, config.clone(), &cname).await?;

                if !group.is_empty().await? {
                    group.commit().await?;
                }
                info!("Committed entry group");
            }
        }
    }

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

async fn add_config(
    server: &ServerProxy<'_>,
    group: &EntryGroupProxy<'_>,
    Config { aliases, services }: Config,
    cname: &Cname,
) -> Result<(), zbus::Error> {
    for alias in aliases {
        add_alias(server, group, alias, cname).await?;
    }

    for (
        name,
        ServiceConfig {
            alias,
            kind,
            protocol,
            port,
        },
    ) in services
    {
        if let Some(alias) = alias {
            add_alias(server, group, alias, cname).await?;
        }

        let service = Service::new(name, kind, protocol, port);
        entry_group_add_service(group, &service).await?;
        info!("Published Service: {service}")
    }

    Ok(())
}

async fn add_alias(
    server: &ServerProxy<'_>,
    group: &EntryGroupProxy<'_>,
    alias: NameBuf,
    cname: &Cname,
) -> Result<(), zbus::Error> {
    if let Some(response) = server_resolve_name(server, &alias).await {
        let owner = response.name;
        if owner != cname.to_string() {
            error!("Entry {alias} already owned by {owner}")
        } else {
            info!("Entry {alias} already published")
        }
    } else {
        let record = Record::new(alias, Ttl::MINUTE, cname);
        entry_group_add_record(group, &record).await?;
        info!("Published Entry: {}", record.name);
    }

    Ok(())
}
