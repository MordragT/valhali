use std::time::Duration;

use avahi_zbus::{
    DnsClass, EntryGroupProxy, EntryGroupState, Protocol, ResolveHostNameResponse, ServerProxy,
    ServerState, Ttl,
};
use name::Name;
use rdata::RecordData;
use tokio::{task::JoinHandle, time};
use zbus::{export::futures_util::StreamExt, zvariant::Optional};

pub mod name;
pub mod rdata;
pub mod status;

pub async fn entry_group_event_handler(
    group: &EntryGroupProxy<'_>,
    f: impl Fn(&EntryGroupState, &str) + Send + Sync + 'static,
) -> JoinHandle<Result<(), zbus::Error>> {
    let rx = group.receive_state_changed().await;

    tokio::spawn(async move {
        let mut rx = rx?;

        while let Some(signal) = rx.next().await {
            let args = signal.args()?;

            let state = args.state();
            let error = args.error();
            f(state, error);
        }

        Ok(())
    })
}

pub async fn entry_group_add_record<D>(
    group: &EntryGroupProxy<'_>,
    name: &Name,
    ttl: Ttl,
    data: &D,
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
) -> JoinHandle<Result<(), zbus::Error>> {
    let rx = server.receive_state_changed().await;

    tokio::spawn(async move {
        let mut rx = rx?;

        while let Some(signal) = rx.next().await {
            let args = signal.args()?;

            let state = args.state();
            let error = args.error();
            f(state, error);
        }

        Ok(())
    })
}

pub async fn server_resolve_name(
    server: &ServerProxy<'_>,
    name: &Name,
) -> Option<ResolveHostNameResponse> {
    match time::timeout(
        // https://github.com/avahi/avahi/blob/master/avahi-core/resolve-service.c#L36
        // #define TIMEOUT_MSEC 5000
        Duration::from_secs(5),
        server.resolve_host_name(
            Optional::default(),
            Protocol::Unspec,
            &name.to_string(),
            Protocol::Unspec,
            0,
        ),
    )
    .await
    {
        Ok(Ok(response)) => Some(response),
        _ => None,
    }
}
