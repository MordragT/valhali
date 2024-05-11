use avahi_zbus::{ServerProxy, ServerState};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ServerStatus {
    pub host_name: String,
    pub domain_name: String,
    pub version: String,
    pub api: u32,
    pub state: ServerState,
}

impl ServerStatus {
    pub async fn from_server(server: &ServerProxy<'_>) -> zbus::Result<Self> {
        let host_name = server.get_host_name().await?;
        let domain_name = server.get_domain_name().await?;
        let version = server.get_version_string().await?;
        let api = server.get_api_version().await?;
        let state = server.get_state().await?;

        Ok(Self {
            host_name,
            domain_name,
            version,
            api,
            state,
        })
    }
}

impl fmt::Display for ServerStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            host_name,
            domain_name,
            version,
            api,
            state,
        } = self;

        write!(
            f,
            "Host: {host_name}\nDomain: {domain_name}\nVersion: {version}\nApi: {api}\nState: {state:?}"
        )
    }
}
