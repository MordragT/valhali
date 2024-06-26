//! # D-Bus interface proxy for: `org.freedesktop.Avahi.RecordBrowser`
//!
//! This code was generated by `zbus-xmlgen` `4.1.0` from D-Bus introspection data.
//! Source: `org.freedesktop.Avahi.RecordBrowser.xml`.
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

use crate::{DnsClass, DnsType, InterfaceIndex, Protocol};
#[proxy(
    interface = "org.freedesktop.Avahi.RecordBrowser",
    default_service = "org.freedesktop.Avahi"
)]
pub trait RecordBrowser {
    /// Free method
    fn free(&self) -> zbus::Result<()>;

    /// Start method
    fn start(&self) -> zbus::Result<()>;

    /// AllForNow signal
    #[zbus(signal)]
    fn all_for_now(&self) -> zbus::Result<()>;

    /// CacheExhausted signal
    #[zbus(signal)]
    fn cache_exhausted(&self) -> zbus::Result<()>;

    /// Failure signal
    #[zbus(signal)]
    fn failure(&self, error: &str) -> zbus::Result<()>;

    /// ItemNew signal
    #[zbus(signal)]
    fn item_new(
        &self,
        interface: Optional<InterfaceIndex>,
        protocol: Protocol,
        name: &str,
        clazz: DnsClass,
        type_: DnsType,
        rdata: Vec<u8>,
        flags: u32,
    ) -> zbus::Result<()>;

    /// ItemRemove signal
    #[zbus(signal)]
    fn item_remove(
        &self,
        interface: Optional<InterfaceIndex>,
        protocol: Protocol,
        name: &str,
        clazz: DnsClass,
        type_: DnsType,
        rdata: Vec<u8>,
        flags: u32,
    ) -> zbus::Result<()>;
}
