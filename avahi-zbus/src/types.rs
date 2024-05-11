use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use zbus::zvariant::{NoneValue, Optional, Type};

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Type, Deserialize_repr, Serialize_repr, Hash,
)]
#[repr(i32)]
pub enum Protocol {
    Unspec = -1,
    Inet = 0,
    Inet6 = 1,
}

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Type, Deserialize, Serialize, Hash,
)]
#[zvariant(signature = "i")]
pub struct InterfaceIndex(pub u16);

impl NoneValue for InterfaceIndex {
    type NoneType = i32;

    fn null_value() -> Self::NoneType {
        -1
    }
}

impl From<i32> for InterfaceIndex {
    fn from(value: i32) -> Self {
        Self(value.try_into().unwrap())
    }
}

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Type, Deserialize_repr, Serialize_repr, Hash,
)]
#[repr(i32)]
pub enum ServerState {
    Invalid,
    Registering,
    Running,
    Collision,
    Failure,
}

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Type, Deserialize_repr, Serialize_repr, Hash,
)]
#[repr(i32)]
pub enum EntryGroupState {
    Uncommitted,
    Registering,
    Established,
    Collision,
    Failure,
}

#[derive(Debug, Clone, Copy)]
pub struct PublishFlags;

impl PublishFlags {
    pub const UNIQUE: i32 = 1;
    pub const NO_PROBE: i32 = 2;
    pub const NO_ANNOUNCE: i32 = 4;
    pub const ALLOW_MULTIPLE: i32 = 8;
    pub const NO_REVERSE: i32 = 16;
    pub const NO_COOKIE: i32 = 32;
    pub const UPDATE: i32 = 64;
    pub const USE_WIDE_AREA: i32 = 128;
    pub const USE_MULTICAST: i32 = 256;
}

#[derive(Debug, Clone, Copy)]
pub struct LookupFlags;

impl LookupFlags {
    pub const USE_WIDE_AREA: i32 = 1;
    pub const USE_MULTICAST: i32 = 2;
    pub const NO_TXT: i32 = 4;
    pub const NO_ADDRESS: i32 = 8;
}

#[derive(Debug, Clone, Copy)]
pub struct LookupResultFlags;

impl LookupResultFlags {
    pub const CACHED: i32 = 1;
    pub const WIDE_AREA: i32 = 2;
    pub const MULTICAST: i32 = 4;
    pub const LOCAL: i32 = 8;
    pub const OUR_OWN: i32 = 16;
    pub const STATIC: i32 = 32;
}

// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Type, Deserialize, Serialize)]
// #[repr(i32)]
// pub enum BrowserEvent {
//     New,
//     Remove,
//     CacheExhausted,
//     AllForNow,
//     Failure,
// }

// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Type, Deserialize, Serialize)]
// #[repr(i32)]
// pub enum ResolverEvent {
//     Found,
//     Failure,
// }

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Type, Deserialize_repr, Serialize_repr, Hash,
)]
#[repr(i32)]
pub enum DomainBrowserType {
    Browse,
    BrowseDefault,
    Register,
    RegisterDefault,
    BrowseLegacy,
    Max,
}

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Type, Deserialize_repr, Serialize_repr, Hash,
)]
#[repr(u16)]
pub enum DnsClass {
    /// Internet (IN).
    /// This class is defined in RFC 1035 and really the only one relevant at all.
    IN = 0x01,
}

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Type, Deserialize_repr, Serialize_repr, Hash,
)]
#[repr(u16)]
pub enum DnsType {
    A = 0x01,
    NS = 0x02,
    CNAME = 0x05,
    SOA = 0x06,
    PTR = 0x0C,
    HINFO = 0x0D,
    MX = 0x0F,
    TXT = 0x10,
    AAAA = 0x1C,
    SRV = 0x21,
}

#[derive(Debug, PartialEq, Eq, Clone, Type, Deserialize, Serialize, Hash)]
pub struct ResolveHostNameResponse {
    pub interface: Optional<InterfaceIndex>,
    pub protocol: Protocol,
    pub name: String,
    pub aprotocol: Protocol,
    pub address: String,
    pub flags: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Type, Deserialize, Serialize, Hash)]
pub struct ResolveAddressResponse {
    pub interface: Optional<InterfaceIndex>,
    pub protocol: Protocol,
    pub aprotocol: Protocol,
    pub address: String,
    pub name: String,
    pub flags: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Type, Deserialize, Serialize, Hash)]
pub struct ResolveServiceResponse {
    pub interface: Optional<InterfaceIndex>,
    pub protocol: Protocol,
    pub name: String,
    pub _type: String,
    pub domain: String,
    pub host: String,
    pub aprotocol: Protocol,
    pub address: String,
    pub port: u16,
    pub txt: Vec<Vec<u8>>,
    pub flags: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Type, Deserialize, Serialize, Hash)]
pub struct Ttl(u32);

const SECS_PER_MINUTE: u32 = 60;
const SECS_PER_HOUR: u32 = 3600;
const SECS_PER_DAY: u32 = 86400;

impl Ttl {
    pub const DEFAULT: Ttl = Ttl::from_mins(75);

    pub const DEFAULT_HOST_NAME: Ttl = Ttl::from_mins(2);

    /// A time-to-live of one second.
    pub const SECOND: Ttl = Ttl::from_secs(1);

    /// A time-to-live of one minute.
    pub const MINUTE: Ttl = Ttl::from_mins(1);

    /// A time-to-live of one hour.
    pub const HOUR: Ttl = Ttl::from_hours(1);

    /// A time-to-live of one day.
    pub const DAY: Ttl = Ttl::from_days(1);

    /// A duration of zero time.
    pub const ZERO: Ttl = Ttl::from_secs(0);

    /// The maximum theoretical time to live.
    pub const MAX: Ttl = Ttl::from_secs(u32::MAX);

    /// The practical maximum time to live as recommended by [RFC 8767](https://datatracker.ietf.org/doc/html/rfc8767#section-4).
    pub const CAP: Ttl = Ttl::from_secs(604_800);

    #[must_use]
    #[inline]
    pub const fn as_secs(&self) -> u32 {
        self.0
    }

    #[must_use]
    #[inline]
    pub const fn as_mins(&self) -> u32 {
        self.0 / SECS_PER_MINUTE
    }

    #[must_use]
    #[inline]
    pub const fn as_hours(&self) -> u32 {
        self.0 / SECS_PER_HOUR
    }

    #[must_use]
    #[inline]
    pub const fn as_days(&self) -> u32 {
        self.0 / SECS_PER_DAY
    }

    /// Creates a new `Ttl` from the specified number of seconds.
    #[must_use]
    #[inline]
    pub const fn from_secs(secs: u32) -> Self {
        Self(secs)
    }

    /// Creates a new `Ttl` from the specified number of minutes.
    ///
    /// # Panics
    ///
    /// The maximum number of days that a `Ttl` can represent is `71582788`.
    /// This method will panic if it is being called with a value greater than that.
    #[must_use]
    #[inline]
    pub const fn from_mins(minutes: u32) -> Self {
        assert!(minutes <= 71582788);
        Self(minutes * SECS_PER_MINUTE)
    }

    /// Creates a new `Ttl` from the specified number of hours.
    ///
    /// # Panics
    ///
    /// The maximum number of hours that a `Ttl` can represent is `1193046`.
    /// This method will panic if it is being called with a value greater than that.
    #[must_use]
    #[inline]
    pub const fn from_hours(hours: u32) -> Self {
        assert!(hours <= 1193046);
        Self(hours * SECS_PER_HOUR)
    }

    /// Creates a new `Ttl` from the specified number of days.
    ///
    /// # Panics
    ///
    /// The maximum number of days that a `Ttl` can represent is `49710`.
    /// This method will panic if it is being called with a value greater than that.
    #[must_use]
    #[inline]
    pub const fn from_days(days: u16) -> Self {
        assert!(days <= 49710);
        Self(days as u32 * SECS_PER_DAY)
    }

    /// Converts a `Ttl` into a [`std::time::Duration`].
    #[must_use]
    #[inline]
    pub const fn into_duration(&self) -> Duration {
        Duration::from_secs(self.0 as u64)
    }
}
