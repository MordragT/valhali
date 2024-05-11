use avahi_zbus::{
    DnsClass, EntryGroupProxy, EntryGroupState, Protocol, ServerProxy, ServerState, Ttl,
};
use name::Name;
use rdata::RecordData;
use zbus::{export::futures_util::StreamExt, zvariant::Optional};

pub mod name;
pub mod rdata;
pub mod status;

pub async fn entry_group_event_handler(
    group: &EntryGroupProxy<'_>,
    f: impl Fn(&EntryGroupState, &str) + Send + Sync + 'static,
) -> Result<(), zbus::Error> {
    let mut group_events = group.receive_state_changed().await?;
    tokio::spawn(async move {
        while let Some(signal) = group_events.next().await {
            let args = signal.args()?;

            let state = args.state();
            let error = args.error();
            f(state, error);
        }

        Ok::<(), zbus::Error>(())
    });

    Ok(())
}

pub async fn entry_group_add_record<D>(
    group: &EntryGroupProxy<'_>,
    name: &Name,
    ttl: Ttl,
    data: D,
) -> Result<(), zbus::Error>
where
    D: RecordData,
{
    group
        .add_record(
            Optional::default(),
            Protocol::Unspec,
            0,
            &name.to_string(),
            DnsClass::IN,
            D::KIND,
            ttl,
            data.as_rdata(),
        )
        .await
}

pub async fn server_event_handler(
    server: &ServerProxy<'_>,
    f: impl Fn(&ServerState, &str) + Send + Sync + 'static,
) -> Result<(), zbus::Error> {
    let mut server_events = server.receive_state_changed().await?;
    tokio::spawn(async move {
        while let Some(signal) = server_events.next().await {
            let args = signal.args()?;

            let state = args.state();
            let error = args.error();
            f(state, error);
        }

        Ok::<(), zbus::Error>(())
    });

    Ok(())
}
