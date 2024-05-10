use avahi_zbus::{DnsType, EntryGroupProxy, Protocol, ServerProxy, DNS_CLASS_IN};
use clap::Parser;
use std::{path::PathBuf, time::Duration};
use tokio::time;
use tracing::{debug, info};
use zbus::{zvariant::Optional, Connection};

#[derive(Parser)]
struct App {
    config: PathBuf,
}

pub const PERIOD: Duration = Duration::from_secs(2);

mod encode;

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

    let fqdn = server.get_host_name_fqdn().await?;
    let rdata = encode::encode_rdata(&fqdn);
    info!("Starting publishing of cnames for `{fqdn}`");

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
                DNS_CLASS_IN,
                DnsType::CNAME,
                60,
                rdata.as_slice(),
            )
            .await?;
        group.commit().await?;

        let state = group.get_state().await?;
        debug!("{state:?} cname record: `vault.local`");

        interval.tick().await;
    }
}
