//! # D-Bus interface proxy for: `org.freedesktop.Avahi.Server2`
//!
//! This code was generated by `zbus-xmlgen` `4.1.0` from D-Bus introspection data.
//! Source: `org.freedesktop.Avahi.Server.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the [Writing a client proxy] section of the zbus
//! documentation.
//!
//! This type implements the [D-Bus standard interfaces], (`org.freedesktop.DBus.*`) for which the
//! following zbus API can be used:
//!
//! * [`zbus::fdo::IntrospectableProxy`]
//!
//! Consequently `zbus-xmlgen` did not generate code for the above interfaces.
//!
//! [Writing a client proxy]: https://dbus2.github.io/zbus/client.html
//! [D-Bus standard interfaces]: https://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces,
use zbus::{proxy, zvariant::Optional};

use crate::{Interface, Protocol};
#[proxy(
    interface = "org.freedesktop.Avahi.Server2",
    default_service = "org.freedesktop.Avahi",
    default_path = "/"
)]
pub trait Server2 {
    /// AddressResolverPrepare method
    fn address_resolver_prepare(
        &self,
        interface: Optional<Interface>,
        protocol: Protocol,
        address: &str,
        flags: u32,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// DomainBrowserPrepare method
    fn domain_browser_prepare(
        &self,
        interface: Optional<Interface>,
        protocol: Protocol,
        domain: &str,
        btype: i32,
        flags: u32,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// EntryGroupNew method
    fn entry_group_new(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// GetAPIVersion method
    #[zbus(name = "GetAPIVersion")]
    fn get_apiversion(&self) -> zbus::Result<u32>;

    /// GetAlternativeHostName method
    fn get_alternative_host_name(&self, name: &str) -> zbus::Result<String>;

    /// GetAlternativeServiceName method
    fn get_alternative_service_name(&self, name: &str) -> zbus::Result<String>;

    /// GetDomainName method
    fn get_domain_name(&self) -> zbus::Result<String>;

    /// GetHostName method
    fn get_host_name(&self) -> zbus::Result<String>;

    /// GetHostNameFqdn method
    fn get_host_name_fqdn(&self) -> zbus::Result<String>;

    /// GetLocalServiceCookie method
    fn get_local_service_cookie(&self) -> zbus::Result<u32>;

    /// GetNetworkInterfaceIndexByName method
    fn get_network_interface_index_by_name(&self, name: &str) -> zbus::Result<i32>;

    /// GetNetworkInterfaceNameByIndex method
    fn get_network_interface_name_by_index(&self, index: i32) -> zbus::Result<String>;

    /// GetState method
    fn get_state(&self) -> zbus::Result<i32>;

    /// GetVersionString method
    fn get_version_string(&self) -> zbus::Result<String>;

    /// HostNameResolverPrepare method
    fn host_name_resolver_prepare(
        &self,
        interface: Optional<Interface>,
        protocol: Protocol,
        name: &str,
        aprotocol: Protocol,
        flags: u32,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// IsNSSSupportAvailable method
    #[zbus(name = "IsNSSSupportAvailable")]
    fn is_nsssupport_available(&self) -> zbus::Result<bool>;

    /// RecordBrowserPrepare method
    #[allow(clippy::too_many_arguments)]
    fn record_browser_prepare(
        &self,
        interface: Optional<Interface>,
        protocol: Protocol,
        name: &str,
        clazz: u16,
        type_: u16,
        flags: u32,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// ResolveAddress method
    #[allow(clippy::too_many_arguments)]
    fn resolve_address(
        &self,
        interface: Optional<Interface>,
        protocol: Protocol,
        address: &str,
        flags: u32,
    ) -> zbus::Result<(i32, i32, i32, String, String, u32)>;

    /// ResolveHostName method
    #[allow(clippy::too_many_arguments)]
    fn resolve_host_name(
        &self,
        interface: Optional<Interface>,
        protocol: Protocol,
        name: &str,
        aprotocol: Protocol,
        flags: u32,
    ) -> zbus::Result<(i32, i32, String, i32, String, u32)>;

    /// ResolveService method
    #[allow(clippy::too_many_arguments)]
    fn resolve_service(
        &self,
        interface: Optional<Interface>,
        protocol: Protocol,
        name: &str,
        type_: &str,
        domain: &str,
        aprotocol: Protocol,
        flags: u32,
    ) -> zbus::Result<(
        i32,
        i32,
        String,
        String,
        String,
        String,
        i32,
        String,
        u16,
        Vec<Vec<u8>>,
        u32,
    )>;

    /// ServiceBrowserPrepare method
    fn service_browser_prepare(
        &self,
        interface: Optional<Interface>,
        protocol: Protocol,
        type_: &str,
        domain: &str,
        flags: u32,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// ServiceResolverPrepare method
    #[allow(clippy::too_many_arguments)]
    fn service_resolver_prepare(
        &self,
        interface: Optional<Interface>,
        protocol: Protocol,
        name: &str,
        type_: &str,
        domain: &str,
        aprotocol: Protocol,
        flags: u32,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// ServiceTypeBrowserPrepare method
    fn service_type_browser_prepare(
        &self,
        interface: Optional<Interface>,
        protocol: Protocol,
        domain: &str,
        flags: u32,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// SetHostName method
    fn set_host_name(&self, name: &str) -> zbus::Result<()>;

    /// StateChanged signal
    #[zbus(signal)]
    fn state_changed(&self, state: i32, error: &str) -> zbus::Result<()>;
}
