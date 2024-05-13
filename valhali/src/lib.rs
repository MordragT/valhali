use std::time::Duration;

use avahi_zbus::{
    DnsClass, EntryGroupProxy, EntryGroupState, Protocol, ResolveHostNameResponse, ServerProxy,
    ServerState, Ttl,
};
use name::Name;
use rdata::RecordData;
use record::Record;
use service::Service;
use tokio::{task::JoinHandle, time};
use zbus::{export::futures_util::StreamExt, zvariant::Optional};

pub mod name;
pub mod rdata;
pub mod record;
pub mod service;
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
    record: &Record<D>,
) -> Result<(), zbus::Error>
where
    D: RecordData,
{
    group
        .add_record(
            Optional::default(),
            Protocol::Unspec,
            0,
            &record.name.to_string(),
            DnsClass::IN,
            D::KIND,
            record.ttl,
            record.data.as_rdata(),
        )
        .await
}

pub async fn entry_group_add_service(
    group: &EntryGroupProxy<'_>,
    service: &Service,
) -> Result<(), zbus::Error> {
    let ty = format!("_{}._{}", service.kinds[0].as_str(), service.protocol);

    group
        .add_service(
            Optional::default(),
            Protocol::Unspec,
            0,
            &service.name,
            &ty,
            "",
            "",
            service.port,
            &[],
        )
        .await?;

    for sub_kind in &service.kinds[1..] {
        let sub_ty = format!("_{sub_kind}");

        group
            .add_service_subtype(
                Optional::default(),
                Protocol::Unspec,
                0,
                &service.name,
                &ty,
                "",
                &sub_ty,
            )
            .await?;
    }

    Ok(())
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
