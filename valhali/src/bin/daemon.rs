use avahi_zbus::{DnsClass, DnsType, EntryGroupProxy, Protocol, ServerProxy, Ttl};
use clap::Parser;
use std::{path::PathBuf, str::FromStr, time::Duration};
use tokio::time;
use tracing::{debug, info};
use valhali::rdata::{Cname, RecordData};
use zbus::{zvariant::Optional, Connection};

#[derive(Parser)]
struct App {
    config: PathBuf,
}

pub const PERIOD: Duration = Duration::from_secs(2);

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

    let cname = Cname::from_str(&server.get_host_name_fqdn().await?)?;
    info!("Starting publishing of aliases for `{cname}`");

    let mut interval = time::interval(PERIOD);

    loop {
        let group_path = server.entry_group_new().await?;
        let group = EntryGroupProxy::new(&connection, group_path).await?;

        group
            .add_record(
                Optional::default(),
                Protocol::Unspec,
                0,
                "vault.local",
                DnsClass::IN,
                DnsType::CNAME,
                Ttl::MINUTE,
                cname.as_rdata(),
            )
            .await?;
        group.commit().await?;

        let state = group.get_state().await?;
        debug!("{state:?} alias: `vault.local`");

        interval.tick().await;
    }
}
